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

struct BlockHeader {
    delta: i128,
    sample1: i128,
    sample2: i128,
    coeff1: i128,
    coeff2: i128,
}

impl BlockHeader {
    pub fn new(block: &[u8], current_channel: usize, total_channels: usize) -> BlockHeader {
        let predictor_index = 0x00 + current_channel;
        let mut predictor: i128 = block[predictor_index] as i128;

        let initial_delta_index = total_channels + current_channel * 0x02;
        let mut delta: i128 = i16::from_le_bytes([block[initial_delta_index], block[initial_delta_index + 0x01]]) as i128;

        let sample1_index = total_channels + total_channels * 0x02 + current_channel * 0x02;
        let mut sample1: i128 = i16::from_le_bytes([block[sample1_index], block[sample1_index + 0x01]]) as i128;
        let sample2_index = sample1_index + total_channels * 0x02;
        let mut sample2: i128 = i16::from_le_bytes([block[sample2_index], block[sample2_index + 0x01]]) as i128;


        let coeff_index = (predictor & 0x07) as usize;
        let coeff1: i128 = MSADPCM_COEFS[coeff_index][0] as i128;
        let coeff2: i128 = MSADPCM_COEFS[coeff_index][1] as i128;

        BlockHeader {
            delta,
            sample1,
            sample2,
            coeff1,
            coeff2,
        }
    }
}


fn decode_byte(block_header: &mut BlockHeader, byte: u8, shift: usize) -> [u8; 2] {
    let mut predictor = 0;
    let nibble: i128 = match shift {
        1 => NIBBLE_TO_INT[(byte & 0xf) as usize] as i128,
        _ => NIBBLE_TO_INT[(byte >> 4) as usize] as i128,
    };

    predictor = ((block_header.sample1 * block_header.coeff1) + (block_header.sample2 * block_header.coeff2)) >> 8;
    predictor = predictor + (nibble * block_header.delta);
    predictor = clamp16(predictor);

    block_header.sample2 = block_header.sample1;
    block_header.sample1 = predictor;
    block_header.delta = ((MSADPCM_STEPS[(nibble & 0xf) as usize] as i128) * block_header.delta) >> 8;
    if block_header.delta < 16 { block_header.delta = 16 };

    return (predictor as i16).to_ne_bytes();
}


fn decode_stereo_block(block: &[u8]) -> Vec<u8> {
    let mut temp_buffer: Vec<u8> = Vec::new();

    let mut left_channel_block_header: BlockHeader = BlockHeader::new(block, 0, 2);
    let mut right_channel_block_header: BlockHeader = BlockHeader::new(block, 1, 2);

    add_u2_to_u8_vec(&mut temp_buffer, (left_channel_block_header.sample2 as i16).to_ne_bytes());
    add_u2_to_u8_vec(&mut temp_buffer, (right_channel_block_header.sample2 as i16).to_ne_bytes());

    add_u2_to_u8_vec(&mut temp_buffer, (left_channel_block_header.sample1 as i16).to_ne_bytes());
    add_u2_to_u8_vec(&mut temp_buffer, (right_channel_block_header.sample1 as i16).to_ne_bytes());


    for index in (0x07 * 2)..block.len() {
        let byte = block[index];

        let left_channel_decoded_block = decode_byte(&mut left_channel_block_header, byte, 0);
        let right_channel_decoded_block = decode_byte(&mut right_channel_block_header, byte, 1);

        add_u2_to_u8_vec(&mut temp_buffer, left_channel_decoded_block);
        add_u2_to_u8_vec(&mut temp_buffer, right_channel_decoded_block);
    }

    temp_buffer
}

fn decode_mono_block(block: &[u8]) -> Vec<u8> {
    let mut temp_buffer: Vec<u8> = Vec::new();

    let mut mono_channel_block_header: BlockHeader = BlockHeader::new(block, 0, 1);

    add_u2_to_u8_vec(&mut temp_buffer, (mono_channel_block_header.sample2 as i16).to_ne_bytes());
    add_u2_to_u8_vec(&mut temp_buffer, (mono_channel_block_header.sample1 as i16).to_ne_bytes());


    for index in (0x07 * 1)..block.len() {
        let byte = block[index];

        for shift in 0..2 {
            let mono_channel_decoded_block = decode_byte(&mut mono_channel_block_header, byte, shift);
            add_u2_to_u8_vec(&mut temp_buffer, mono_channel_decoded_block);
        }
    }

    temp_buffer
}

pub fn decode(metadata: &Metadata, buffer: &mut Buffer) -> Vec<u8> {
    let offset: usize = metadata.audio_offset as usize;
    let frame_size: usize = metadata.entry_frame_size as usize;
    let mut output_buffer: Vec<u8> = Vec::new();

    let mut decoded: usize = 0;
    let length = metadata.entry_stream_size as usize;


    if metadata.entry_channels == 1 {
        while decoded < length {
            let mut current_frame_size = frame_size;
            if decoded + frame_size > length {
                current_frame_size = length - decoded;
            }

            let block: &[u8] = buffer.vec(offset + decoded, current_frame_size);
            let mut decoded_block = decode_mono_block(block);
            output_buffer.append(&mut decoded_block);

            decoded += current_frame_size;
        }
    } else if metadata.entry_channels == 2 {
        while decoded < length {
            let mut current_frame_size = frame_size;
            if decoded + frame_size > length {
                current_frame_size = length - decoded;
            }

            let block: &[u8] = buffer.vec(offset + decoded, current_frame_size);
            let mut decoded_block = decode_stereo_block(block);
            output_buffer.append(&mut decoded_block);

            decoded += current_frame_size;
        }
    } else {
        panic!("Unsupported channel count {}", metadata.entry_channels);
    }


    output_buffer
}

fn clamp16(val: i128) -> i128 {
    if val > 32767 {
        return 32767;
    } else if val < -32768 {
        return -32768;
    }
    return val;
}


fn add_u2_to_u8_vec(add: &mut Vec<u8>, with: [u8; 2]) {
    for item in with {
        add.push(item)
    }
}