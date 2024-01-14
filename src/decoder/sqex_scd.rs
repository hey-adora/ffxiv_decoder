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

pub fn decode_block(block: &[u8]) -> Vec<u8> {
    let mut temp_buffer: Vec<u8> = Vec::new();
    let mut predictor: i32 = block[0x00] as i32;
    let mut initial_delta: i32 = i16::from_le_bytes([block[0x01], block[0x02]]) as i32;
    let mut sample1: i32 = i16::from_le_bytes([block[0x03], block[0x04]]) as i32;
    let mut sample2: i32 = i16::from_le_bytes([block[0x05], block[0x06]]) as i32;
    add_u2_to_u8_vec(&mut temp_buffer, (sample2 as i16).to_ne_bytes());
    add_u2_to_u8_vec(&mut temp_buffer, (sample1 as i16).to_ne_bytes());
    let coeff_index = (predictor & 0x07) as usize;
    let coeff1: i32 = MSADPCM_COEFS[coeff_index][0] as i32;
    let coeff2: i32 = MSADPCM_COEFS[coeff_index][1] as i32;


    for index in 0x07..block.len() {
        let byte = block[index];
        for shift in 0..2 {
            let nibble: i32 = match shift {
                1 => NIBBLE_TO_INT[(byte & 0xf) as usize] as i32,
                _ => NIBBLE_TO_INT[(byte >> 4) as usize] as i32,
            };
            predictor = ((sample1 * coeff1) + (sample2 * coeff2)) >> 8;
            predictor = predictor + (nibble * initial_delta);
            predictor = clamp16(predictor);
            add_u2_to_u8_vec(&mut temp_buffer, (predictor as i16).to_ne_bytes());
            sample2 = sample1;
            sample1 = predictor;
            initial_delta = ((MSADPCM_STEPS[(nibble & 0xf) as usize] as i32) * initial_delta) >> 8;
            if initial_delta < 16 { initial_delta = 16 };
        }
    }

    temp_buffer
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

        let block: &[u8] = buffer.vec(offset + decoded, current_frame_size);
        let mut decoded_block = decode_block(block);
        output_buffer.append(&mut decoded_block);

        decoded += current_frame_size;
    }

    output_buffer
}

fn clamp16(val: i32) -> i32 {
    if val > 32767 {
        return 32767;
    } else if val < -32768 {
        return -32768;
    }
    return val;
}