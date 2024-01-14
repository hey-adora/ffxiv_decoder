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

// fn get_frame(offset: usize, buffer: &mut Buffer, frame_size: usize) -> Vec<u8> {
//     let mut frame_count: usize = 2;
//     let frame_offset_start = offset + 70 * (frame_count - 1);
//     let frame_offset_end = offset + 70 * frame_count;
//     let frame_buffer_size = frame_offset_end - frame_offset_start;
//     let mut frame: Vec<u8> = vec![0; frame_buffer_size];
//     frame.copy_from_slice(buffer.vec(frame_offset_start, frame_buffer_size));
//     return frame;
// }

// fn decode_frame2(frame: &[u8], output_buffer: &mut Vec<u8>) {
//     let mut adpcm_coef: [i16; 16] = [0; 16];
//     let mut adpcm_scale: i32 = i16::from_ne_bytes([frame[0x01], frame[0x02]]) as i32;
//     let mut adpcm_history1_16: i16 = i16::from_ne_bytes([frame[0x03], frame[0x04]]);
//     let mut adpcm_history2_16: i16 = i16::from_ne_bytes([frame[0x05], frame[0x06]]);
//
//     let index: usize = (frame[0] & 0x07) as usize;
//     adpcm_coef[0] = MSADPCM_COEFS[index][0];
//     adpcm_coef[1] = MSADPCM_COEFS[index][1];
//
//     add_u2_to_u8_vec(output_buffer, adpcm_history2_16.to_ne_bytes());
//     add_u2_to_u8_vec(output_buffer, adpcm_history1_16.to_ne_bytes());
//
//     for index in 0x07..frame.len() {
//         //let shift = index & 1;
//         for shift in 0..2 {
//             let hex = frame[index];
//
//             let predicted: i16 = msadpcm_adpcm_expand_nibble_div(
//                 &mut adpcm_scale,
//                 &mut adpcm_history1_16,
//                 &mut adpcm_history2_16,
//                 adpcm_coef,
//                 hex,
//                 shift,
//             );
//
//             add_u2_to_u8_vec(output_buffer, predicted.to_ne_bytes());
//         }
//     }
// }

static mut g: i32 = 0;

pub fn decode_frame(block: &[u8], output_buffer: &mut Vec<u8>) {
    let mut predictor: i32 = block[0x00] as i32;
    let mut initial_delta: i32 = i16::from_le_bytes([block[0x01], block[0x02]]) as i32;
    let mut sample1: i32 = i16::from_le_bytes([block[0x03], block[0x04]]) as i32;
    let mut sample2: i32 = i16::from_le_bytes([block[0x05], block[0x06]]) as i32;
    add_u2_to_u8_vec(output_buffer, (sample2 as i16).to_ne_bytes());
    add_u2_to_u8_vec(output_buffer, (sample1 as i16).to_ne_bytes());
    let coeff_index = (predictor & 0x07) as usize;
    let coeff1: i32 = MSADPCM_COEFS[coeff_index][0] as i32;
    let coeff2: i32 = MSADPCM_COEFS[coeff_index][1] as i32;


    for index in 0x07..block.len() {
        let byte = block[index];
        for index2 in 1..3 {
            let shift = index2 & 1;
            let nibble: i32 = match shift {
                1 => NIBBLE_TO_INT[(byte >> 4) as usize] as i32,
                _ => NIBBLE_TO_INT[(byte & 0xf) as usize] as i32,
            };
            predictor = ((sample1 * coeff1) + (sample2 * coeff2)) >> 8;
            predictor = predictor + (nibble * initial_delta);
            predictor = clamp16(predictor);
            add_u2_to_u8_vec(output_buffer, (predictor as i16).to_ne_bytes());
            sample2 = sample1;
            sample1 = predictor;
            initial_delta = ((MSADPCM_STEPS[(nibble & 0xf) as usize] as i32) * initial_delta) >> 8;
            if initial_delta < 16 { initial_delta = 16 };
            unsafe {
                g += 1;
                if predictor == -20 {
                    println!("test {}", g);
                    println!("test");
                }
            }
        }
    }
}

pub fn decode(metadata: Metadata, buffer: &mut Buffer) -> Vec<u8> {
    let offset: usize = metadata.audio_offset as usize;
    let frame_size: usize = metadata.entry_frame_size as usize;
    let mut output_buffer: Vec<u8> = Vec::new();

    let mut decoded: usize = 0;
    let length = metadata.entry_stream_size as usize;


    while decoded < length {
        let mut current_frame_size = frame_size;
        if decoded + frame_size > length {
            current_frame_size = length - decoded;
        }

        let mut block: &[u8] = buffer.vec(offset + decoded, current_frame_size);
        decode_frame(block, &mut output_buffer);
        //decode_frame2(block, &mut output_buffer);

        decoded += current_frame_size;
    }

    output_buffer
}

// fn msadpcm_adpcm_expand_nibble_div(
//     adpcm_scale: &mut i32,
//     adpcm_history1_16: &mut i16,
//     adpcm_history2_16: &mut i16,
//     adpcm_coef: [i16; 16],
//     hex: u8,
//     shift: usize,
// ) -> i16 {
//     let mut code: i32 = 0;
//     if shift != 1 {
//         code = *NIBBLE_TO_INT
//             .get((hex >> 4) as usize)
//             .expect("failed to get_high_nibble_signed") as i32;
//     } else {
//         code = *NIBBLE_TO_INT
//             .get((hex & 0x0f) as usize)
//             .expect("failed to get_low_nibble_signed") as i32;
//     }
//
//     let his1: i32 = *adpcm_history1_16 as i32;
//     let his2: i32 = *adpcm_history2_16 as i32;
//     let adpcm_coef1: i32 = *adpcm_coef.get(0).expect("Failed to get adpcm_coef[0]") as i32;
//     let adpcm_coef2: i32 = *adpcm_coef.get(1).expect("Failed to get adpcm_coef[1]") as i32;
//
//     let mut predicted: i32 = his1 * adpcm_coef1 + adpcm_coef2 * his2;
//     predicted = predicted / 256;
//     predicted = predicted + code * *adpcm_scale;
//     predicted = clamp16(predicted);
//
//     *adpcm_history2_16 = *adpcm_history1_16;
//     *adpcm_history1_16 = predicted as i16;
//
//     let adpcm_scale_step: i32 = *MSADPCM_STEPS
//         .get((code & 0x0f) as usize)
//         .expect("Failed to get msadpcm_steps for adpcm_scale") as i32;
//     *adpcm_scale = (adpcm_scale_step * *adpcm_scale) / 256;
//
//     if *adpcm_scale < 16 {
//         *adpcm_scale = 16;
//     }
//
//     return predicted as i16;
// }

fn clamp16(val: i32) -> i32 {
    if val > 32767 {
        return 0;
    } else if val < -32768 {
        return -32768;
    }
    return val;
}