pub mod parser;
pub mod reader;
pub mod decoder;
pub mod visualizer;

// pub mod scd_parser {
//     pub use crate::parser::reader;
//     use std::fs::File;
//     use std::io::{BufReader, Read};
//


// impl SqexScd {
//     fn msadpcm_bytes_to_samples(stream_size: i32, frame_size: i32, channels: i32) -> i32 {
//         if frame_size <= 0 || channels <= 0 {
//             return 0;
//         } else {
//             let fraction = stream_size % frame_size;
//             let mut add = 0;
//             if fraction != 0 {
//                 add = (fraction - (7 - 1) * channels) * 2 / channels
//             }
//             let output = (stream_size / frame_size) * (frame_size - (7 - 1) * channels) * 2
//                 / channels
//                 + add;
//             return output;
//         }
//     }
//
//     pub fn read_file(file_path: &str) -> Vec<u8> {
//         let file = File::open(file_path).expect("Failed to open file.");
//         let mut reader = BufReader::new(file);
//         let mut buffer = Vec::new();
//         reader
//             .read_to_end(&mut buffer)
//             .expect("Failed to read file.");
//         buffer
//     }
//
//     pub fn new(buffer: Vec<u8>, print_column_count: usize) -> SqexScd {
//
//
//
//
//         let offset = extradata_size + (entries_offset as i32) + 0x20;
//         let buffer_size_kb = 1;
//         let buffer_size = 1024 * buffer_size_kb;
//
//         let parse_cursor = buffer[0];
//         SqexScd {
//             buffer,
//             signature: HexValue {
//                 value: String::from("test"),
//                 hex_list: vec![0, 0],
//                 start: 0x00,
//                 end: 0x01,
//                 size: 0x01,
//             }, // parse_cursor,
//                // parse_index: 0,
//                // print_column_count,
//                // print_column_index: 0,
//                // print_row_index: 0,
//                // print_row: Vec::new(),
//                // print_marks: Vec::new(),
//         }
//     }
//
//     fn decode_get_samples_per_frame(frame_size: i32, channels: i32) -> i32 {
//         return (frame_size - 0x07 * channels) * 2 / channels + 2;
//     }
//
//     fn decode_get_samples_to_do(
//         samples_this_block: i32,
//         samples_per_frame: i32,
//         samples_into_block: i32,
//     ) -> i32 {
//         let samples_left_this_block = samples_this_block - samples_into_block;
//         if samples_per_frame > 1
//             && (samples_into_block % samples_per_frame) + samples_left_this_block
//                 > samples_per_frame
//         {
//             let samples_to_do = samples_per_frame - (samples_into_block % samples_per_frame);
//             return samples_to_do;
//         }
//         return samples_left_this_block;
//     }
//
//     fn vec_replace_u8_with_u8(replace: &mut Vec<u8>, with: &Vec<u8>, start: usize) {
//         for (index, item) in with.iter().enumerate() {
//             replace[start + index] = *item;
//         }
//     }
//
//     fn add_u2_to_u8_vec(add: &mut Vec<u8>, with: [u8; 2]) {
//         for item in with {
//             add.push(item)
//         }
//     }
//
//     pub fn parse(&mut self) {
//         let samples_num =
//             SqexScd::msadpcm_bytes_to_samples(stream_size, frame_size as i32, channels); //69376
//         let mut samples_into_block: i32 = 0;
//         let mut current_sample: i32 = 0;
//         let max_buffer_samples: i32 = buffer_size / (channels * 2); //512
//         let play_forever: usize = 0;
//
//         let mut buffer: Vec<u8> = Vec::new();
//         let mut decode_pos_samples: i32 = 0;
//         let mut to_do: i32 = 0;
//
//         println!("OFFSET: {}", offset);
//
//         let offset_u: usize = offset as usize;
//
//         let mut frame_count: usize = 4;
//         let frame_offset_start = offset_u + 70 * (frame_count - 1);
//         let frame_offset_end = offset_u + 70 * frame_count;
//         let frame_buffer_size = frame_offset_end - frame_offset_start;
//         let mut frame2: Vec<u8> = vec![0; frame_buffer_size];
//         frame2.copy_from_slice(&self.buffer[frame_offset_start..frame_offset_end]);
//
//         let index: usize = (frame2[0] & 0x07) as usize;
//
//         let mut output_buffer: Vec<u8> = Vec::new();
//         let mut adpcm_coef: [i16; 16] = [0; 16];
//         adpcm_coef[0] = SqexScd::msadpcm_coefs[index][0];
//         adpcm_coef[1] = SqexScd::msadpcm_coefs[index][1];
//         let mut adpcm_scale: i16 = i16::from_ne_bytes([frame2[0x01], frame2[0x02]]);
//         let mut adpcm_history1_16: i16 = i16::from_ne_bytes([frame2[0x03], frame2[0x04]]);
//         let mut adpcm_history2_16: i16 = i16::from_ne_bytes([frame2[0x05], frame2[0x06]]);
//
//         SqexScd::add_u2_to_u8_vec(&mut output_buffer, adpcm_history2_16.to_ne_bytes());
//         SqexScd::add_u2_to_u8_vec(&mut output_buffer, adpcm_history1_16.to_ne_bytes());
//
//         let offet_done: usize = 0x07;
//         let offffffset: usize = frame2.len();
//         for index in 0x07..frame2.len() {
//             for shift in 0..2 {
//                 let hex = frame2[index];
//
//                 let prdicted: i16 = Decoder::msadpcm_adpcm_expand_nibble_shr(
//                     &mut adpcm_scale,
//                     &mut adpcm_history1_16,
//                     &mut adpcm_history2_16,
//                     adpcm_coef,
//                     hex,
//                     shift,
//                 );
//                 SqexScd::add_u2_to_u8_vec(&mut output_buffer, prdicted.to_ne_bytes());
//             }
//         }
//
//         print!("\n\n");
//
//         for hex in frame2 {
//             print!("{:02x} ", hex);
//         }
//
//         print!("\n\n");
//
//         for hex in output_buffer {
//             print!("{:02x} ", hex);
//         }
//
//         println!(
//             "\nHERE, adpcm_1: {}, adpcm_2: {}, adpcm_scale: {}, adpcm_history1_16: {}, adpcm_history2_16: {}",
//             adpcm_coef[0], adpcm_coef[1], adpcm_scale, adpcm_history1_16, adpcm_history2_16
//         );
//
//         println!("{}", frame_size);
//
//         // self.print_column_headers();
//         // for hex in &self.buffer {
//         //     if self.parse_index > 1000 {
//         //         break;
//         //     }
//         //     self.parse_cursor = *hex;
//
//         //     if self.print_column_index < *&self.print_column_count {
//         //         self.print_row.push(self.parse_cursor);
//         //         self.print_column_index += 1;
//         //     } else {
//         //         self.print_row();
//         //         self.print_row.clear();
//         //         self.print_column_index = 0;
//         //         self.print_row_index += self.print_column_count;
//
//         //         self.print_row.push(self.parse_cursor);
//         //         self.print_column_index += 1;
//         //     }
//
//         //     self.parse_index += 1;
//         // }
//     }


//     }
// }
