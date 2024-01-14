use crate::reader::Buffer;
use crate::parser::sqex_scd::Metadata;

const NIBBLE_TO_INT: [i16; 16] = [0, 1, 2, 3, 4, 5, 6, 7, -8, -7, -6, -5, -4, -3, -2, -1];
const MSADPCM_STEPS: [i16; 16] = [
    230, 230, 230, 230, 307, 409, 512, 614, 768, 614, 512, 409, 307, 230, 230, 230,
];
const MSADPCM_COEFS: [[i16; 2]; 7] = [
    [256, 0],
    [512, -256],
    [0, 0],
    [192, 64],
    [240, 0],
    [460, -208],
    [392, -232],
];

fn add_u2_to_u8_vec(add: &mut Vec<u8>, with: [u8; 2]) {
    for item in with {
        add.push(item)
    }
}


pub fn decode(metadata: Metadata, buffer: &mut Buffer) -> Vec<u8> {
    let offset_u: usize = metadata.audio_offset as usize;

    let mut frame_count: usize = 2;
    let frame_offset_start = offset_u + 70 * (frame_count - 1);
    let frame_offset_end = offset_u + 70 * frame_count;
    let frame_buffer_size = frame_offset_end - frame_offset_start;
    let mut frame2: Vec<u8> = vec![0; frame_buffer_size];
    frame2.copy_from_slice(buffer.vec(frame_offset_start, frame_buffer_size));

    let index: usize = (frame2[0] & 0x07) as usize;

    let mut output_buffer: Vec<u8> = Vec::new();
    let mut adpcm_coef: [i16; 16] = [0; 16];
    adpcm_coef[0] = MSADPCM_COEFS[index][0];
    adpcm_coef[1] = MSADPCM_COEFS[index][1];
    let mut adpcm_scale: i16 = i16::from_ne_bytes([frame2[0x01], frame2[0x02]]);
    let mut adpcm_history1_16: i16 = i16::from_ne_bytes([frame2[0x03], frame2[0x04]]);
    let mut adpcm_history2_16: i16 = i16::from_ne_bytes([frame2[0x05], frame2[0x06]]);

    add_u2_to_u8_vec(&mut output_buffer, adpcm_history2_16.to_ne_bytes());
    add_u2_to_u8_vec(&mut output_buffer, adpcm_history1_16.to_ne_bytes());

    let offet_done: usize = 0x07;
    let offffffset: usize = frame2.len();
    for index in 0x07..frame2.len() {
        for shift in 0..2 {
            let hex = frame2[index];

            let prdicted: i16 = msadpcm_adpcm_expand_nibble_shr(
                &mut adpcm_scale,
                &mut adpcm_history1_16,
                &mut adpcm_history2_16,
                adpcm_coef,
                hex,
                shift,
            );
            add_u2_to_u8_vec(&mut output_buffer, prdicted.to_ne_bytes());
        }
    }

    output_buffer
}

fn msadpcm_adpcm_expand_nibble_shr(
    adpcm_scale: &mut i16,
    adpcm_history1_16: &mut i16,
    adpcm_history2_16: &mut i16,
    adpcm_coef: [i16; 16],
    hex: u8,
    shift: usize,
) -> i16 {
    let mut code: i16 = 0;
    if shift == 1 {
        code = *NIBBLE_TO_INT
            .get((hex >> 4) as usize)
            .expect("failed to get_high_nibble_signed");
    } else {
        code = *NIBBLE_TO_INT
            .get((hex & 0x0f) as usize)
            .expect("failed to get_low_nibble_signed");
    }

    let adpcm_coef1: i16 = *adpcm_coef.get(0).expect("Failed to get adpcm_coef[0]");
    let adpcm_coef2: i16 = *adpcm_coef.get(1).expect("Failed to get adpcm_coef[1]");

    let mut predicted: i16 =
        *adpcm_history1_16 * adpcm_coef1 + *adpcm_history2_16 * adpcm_coef2;
    predicted = predicted >> 8;
    predicted = predicted + (code * *adpcm_scale);
    predicted = clamp16(predicted);

    *adpcm_history2_16 = *adpcm_history1_16;
    *adpcm_history1_16 = predicted as i16;

    let adpcm_scale_step: i16 = *MSADPCM_STEPS
        .get((code & 0x0f) as usize)
        .expect("Failed to get msadpcm_steps for adpcm_scale");
    *adpcm_scale = (adpcm_scale_step * *adpcm_scale) >> 8;

    if (*adpcm_scale < 16) {
        *adpcm_scale = 16;
    }

    return predicted;
}

fn clamp16(val: i16) -> i16 {
    if (val > 32767) {
        return 32767;
    } else if val < -32768 {
        return -32768;
    }
    return val;
}