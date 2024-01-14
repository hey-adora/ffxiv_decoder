#![allow(dead_code)]
#![allow(unused_variables)]

pub mod scd_parser {
    //use colored::Colorize;
    use std::fs::File;
    use std::io::{BufReader, Read};

    #[derive(Clone, Copy)]
    pub enum PrintColor {
        Default = 9,
        Black = 0,
        Red = 1,
        Green = 2,
        Yellow = 3,
        Blue = 4,
        Magenta = 5,
        Cyan = 6,
        White = 7,
    }

    #[derive(Clone)]
    struct HexMark {
        pub start: usize,
        pub end: usize,
        pub size: usize,
        pub color: PrintColor,
    }

    #[derive(Clone)]
    struct HexValue<T> {
        value: T,
        hex_list: Vec<u8>,
        mark: HexMark,
    }

    #[derive(Clone)]
    pub struct Parser {
        buffer: Vec<u8>,
        parse_cursor: u8,
        parse_index: usize,
        print_column_count: usize,
        print_column_index: usize,
        print_row_index: usize,
        print_row: Vec<u8>,
        print_marks: Vec<HexMark>,
    }

    #[derive(Clone)]
    pub struct PrinterColorPack {
        pub text: PrintColor,
        pub space: PrintColor,
    }

    // pub trait ParseFromU8<T, E> {
    //     fn parse_from_u8(buffer: Vec<u8>) -> Result<T, E>;
    // }

    // impl ParseFromU8<String, std::string::FromUtf8Error> for String {
    //     fn parse_from_u8(buffer: Vec<u8>) -> Result<String, std::string::FromUtf8Error> {
    //         String::from_utf8(buffer)
    //     }
    // }

    // impl ParseFromU8<u8> for u8 {
    //     fn parse_from_u8(buffer: Vec<u8>) -> u8 {
    //         u8::from_ne_bytes(bytes)
    //     }
    // }

    // long msadpcm_bytes_to_samples(long stream_size, int frame_size, int channels) {
    //     if (frame_size <= 0 || channels <= 0) return 0;
    //         return (stream_size / frame_size) * (frame_size - (7-1)*channels) * 2 / channels + ((stream_size % frame_size) ? ((stream_size % frame_size) - (7-1)*channels) * 2 / channels : 0);
    // }

    impl Parser {
        const nibble_to_int: [i16; 16] = [0, 1, 2, 3, 4, 5, 6, 7, -8, -7, -6, -5, -4, -3, -2, -1];
        const msadpcm_steps: [i16; 16] = [
            230, 230, 230, 230, 307, 409, 512, 614, 768, 614, 512, 409, 307, 230, 230, 230,
        ];
        const msadpcm_coefs: [[i16; 2]; 7] = [
            [256, 0],
            [512, -256],
            [0, 0],
            [192, 64],
            [240, 0],
            [460, -208],
            [392, -232],
        ];

        fn msadpcm_bytes_to_samples(stream_size: i32, frame_size: i32, channels: i32) -> i32 {
            if frame_size <= 0 || channels <= 0 {
                return 0;
            } else {
                let fraction = stream_size % frame_size;
                let mut add = 0;
                if fraction != 0 {
                    add = (fraction - (7 - 1) * channels) * 2 / channels
                }
                let output = (stream_size / frame_size) * (frame_size - (7 - 1) * channels) * 2
                    / channels
                    + add;
                return output;
            }
        }

        pub fn read_file(file_path: &str) -> Vec<u8> {
            let file = File::open(file_path).expect("Failed to open file.");
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .expect("Failed to read file.");
            buffer
        }

        pub fn new(buffer: Vec<u8>, print_column_count: usize) -> Parser {
            let parse_cursor = buffer[0];
            Parser {
                buffer,
                parse_cursor,
                parse_index: 0,
                print_column_count,
                print_column_index: 0,
                print_row_index: 0,
                print_row: Vec::new(),
                print_marks: Vec::new(),
            }
        }

        // fn render_vgmstream() -> i32 {

        // }

        fn decode_get_samples_per_frame(frame_size: i32, channels: i32) -> i32 {
            return (frame_size - 0x07 * channels) * 2 / channels + 2;
        }

        fn decode_get_samples_to_do(
            samples_this_block: i32,
            samples_per_frame: i32,
            samples_into_block: i32,
        ) -> i32 {
            let samples_left_this_block = samples_this_block - samples_into_block;
            if samples_per_frame > 1
                && (samples_into_block % samples_per_frame) + samples_left_this_block
                    > samples_per_frame
            {
                //println!("HMMMMMMMMMMMMMMMMMMMMMMMMMM");
                let samples_to_do = samples_per_frame - (samples_into_block % samples_per_frame);
                return samples_to_do;
            }
            return samples_left_this_block;
        }

        fn vec_replace_u8_with_u8(replace: &mut Vec<u8>, with: &Vec<u8>, start: usize) {
            for (index, item) in with.iter().enumerate() {
                replace[start + index] = *item;
            }
        }

        fn add_u2_to_u8_vec(add: &mut Vec<u8>, with: [u8; 2]) {
            for item in with {
                add.push(item)
            }
        }

        fn clamp16(val: i16) -> i16 {
            if (val > 32767) {
                return 32767;
            } else if val < -32768 {
                return -32768;
            }
            return val;
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
                code = *Parser::nibble_to_int
                    .get((hex >> 4) as usize)
                    .expect("failed to get_high_nibble_signed");
            } else {
                code = *Parser::nibble_to_int
                    .get((hex & 0x0f) as usize)
                    .expect("failed to get_low_nibble_signed");
            }

            let adpcm_coef1: i16 = *adpcm_coef.get(0).expect("Failed to get adpcm_coef[0]");
            let adpcm_coef2: i16 = *adpcm_coef.get(1).expect("Failed to get adpcm_coef[1]");

            let mut predicted: i16 =
                *adpcm_history1_16 * adpcm_coef1 + *adpcm_history2_16 * adpcm_coef2;
            predicted = predicted >> 8;
            predicted = predicted + (code * *adpcm_scale);
            predicted = Parser::clamp16(predicted);

            *adpcm_history2_16 = *adpcm_history1_16;
            *adpcm_history1_16 = predicted as i16;

            let adpcm_scale_step: i16 = *Parser::msadpcm_steps
                .get((code & 0x0f) as usize)
                .expect("Failed to get msadpcm_steps for adpcm_scale");
            *adpcm_scale = (adpcm_scale_step * *adpcm_scale) >> 8;

            if (*adpcm_scale < 16) {
                *adpcm_scale = 16;
            }

            return predicted;
        }

        //fn msadpcm_adpcm_expand_nibble_div() -> i16 {}

        pub fn parse(&mut self) {
            let signature = self.string(0x00, 0x08);
            let version = self.i16(0x08);
            let big_endian = self.u8(0x0c);
            let sscf_version = self.u8(0x0d);
            let tables_offset = self.i16(0x0e);
            let size_of_table_0 = self.i16(tables_offset as usize);
            let size_of_sound_entry_offset_table = self.i16((tables_offset + 0x02) as usize);
            let header_entries = self.i16((tables_offset + 0x04) as usize);
            let offset_table_0_offset = self.u32((tables_offset + 0x08) as usize);
            let headers_offset = self.u32((tables_offset + 0x0c) as usize);
            let offset_to_offset_table_2 = self.u32((tables_offset + 0x0c) as usize);
            let entries_offset = self.u32((offset_to_offset_table_2) as usize);
            let stream_size = self.i32((entries_offset) as usize);
            let channels = self.i32((entries_offset + 0x4) as usize);
            let sample_rate = self.i32((entries_offset + 0x8) as usize);
            let codex = self.i32((entries_offset + 0xc) as usize);
            let loop_start = self.i32((entries_offset + 0x10) as usize);
            let loop_end = self.i32((entries_offset + 0x14) as usize);
            let extradata_size = self.i32((entries_offset + 0x18) as usize);
            let aux_chunk_count = self.i32((entries_offset + 0x1c) as usize);
            let extradata_offset = self.i32((entries_offset + 0x20) as usize);
            let frame_size = self.i16((entries_offset + 0x2c) as usize);
            let waveformatex = self.u16((entries_offset + 0x34) as usize);

            let offset = extradata_size + (entries_offset as i32) + 0x20;
            let buffer_size_kb = 1;
            let buffer_size = 1024 * buffer_size_kb;

            //let samples_to_do: i32 = 0;
            //let length_samples: i32 = samples_num
            let samples_num =
                Parser::msadpcm_bytes_to_samples(stream_size, frame_size as i32, channels); //69376
            let mut samples_into_block: i32 = 0;
            let mut current_sample: i32 = 0;
            let max_buffer_samples: i32 = buffer_size / (channels * 2); //512
            let play_forever: usize = 0;

            let mut buffer: Vec<u8> = Vec::new();
            let mut decode_pos_samples: i32 = 0;
            let mut to_do: i32 = 0;

            println!("OFFSET: {}", offset);

            let offset_u: usize = offset as usize;
            // let samples_written_u: usize = samples_written as usize;
            // let frame_size_u: usize = frame_size as usize;

            // let frame_offset_start: usize = offset_u + samples_written_u + decode_pos_samples;
            // let frame_offset_end: usize =
            //     offset_u + frame_size_u + samples_written_u + decode_pos_samples;
            // let frame_buffer_size: usize = frame_offset_end - frame_offset_start;

            // let mut frame: Vec<u8> = vec![0; 70];
            // frame.copy_from_slice(&self.buffer[offset_u..(offset_u + 70)]);

            let mut frame_count: usize = 4;
            let frame_offset_start = offset_u + 70 * (frame_count - 1);
            let frame_offset_end = offset_u + 70 * frame_count;
            let frame_buffer_size = frame_offset_end - frame_offset_start;
            let mut frame2: Vec<u8> = vec![0; frame_buffer_size];
            frame2.copy_from_slice(&self.buffer[frame_offset_start..frame_offset_end]);

            let index: usize = (frame2[0] & 0x07) as usize;

            let mut output_buffer: Vec<u8> = Vec::new();
            let mut adpcm_coef: [i16; 16] = [0; 16];
            adpcm_coef[0] = Parser::msadpcm_coefs[index][0];
            adpcm_coef[1] = Parser::msadpcm_coefs[index][1];
            let mut adpcm_scale: i16 = i16::from_ne_bytes([frame2[0x01], frame2[0x02]]);
            let mut adpcm_history1_16: i16 = i16::from_ne_bytes([frame2[0x03], frame2[0x04]]);
            let mut adpcm_history2_16: i16 = i16::from_ne_bytes([frame2[0x05], frame2[0x06]]);

            //let ttt = adpcm_history2_16.to_ne_bytes();
            //output_buffer[0] = adpcm_history2_16;
            Parser::add_u2_to_u8_vec(&mut output_buffer, adpcm_history2_16.to_ne_bytes());
            Parser::add_u2_to_u8_vec(&mut output_buffer, adpcm_history1_16.to_ne_bytes());

            let offet_done: usize = 0x07;
            let offffffset: usize = frame2.len();
            for index in 0x07..frame2.len() {
                for shift in 0..2 {
                    let hex = frame2[index];

                    let prdicted: i16 = Parser::msadpcm_adpcm_expand_nibble_shr(
                        &mut adpcm_scale,
                        &mut adpcm_history1_16,
                        &mut adpcm_history2_16,
                        adpcm_coef,
                        hex,
                        shift,
                    );
                    Parser::add_u2_to_u8_vec(&mut output_buffer, prdicted.to_ne_bytes());
                    //println!("{}: {}: {}", index, hex, prdicted);
                }
                //let hex_offset: usize = index + 0x07 + (index - 2) / 2;

                // let hex = frame2[hex_offset];
                //  print!("{:02x} ", hex);
            }

            print!("\n\n");

            for hex in frame2 {
                print!("{:02x} ", hex);
            }

            print!("\n\n");

            for hex in output_buffer {
                print!("{:02x} ", hex);
            }

            println!(
                "\nHERE, adpcm_1: {}, adpcm_2: {}, adpcm_scale: {}, adpcm_history1_16: {}, adpcm_history2_16: {}",
                adpcm_coef[0], adpcm_coef[1], adpcm_scale, adpcm_history1_16, adpcm_history2_16
            );
            // loop {
            //     if (decode_pos_samples + max_buffer_samples) > samples_num {
            //         to_do = samples_num - decode_pos_samples;
            //     } else {
            //         to_do = max_buffer_samples;
            //     }
            //     if to_do <= 0 {
            //         break;
            //     }

            //     let mut samples_written: i32 = 0;
            //     let samples_per_frame =
            //         Parser::decode_get_samples_per_frame(frame_size as i32, channels);

            //     println!("TO_DO: {}, done: {}", to_do, decode_pos_samples);
            //     while samples_written < to_do {
            //         let mut samples_to_do: usize = Parser::decode_get_samples_to_do(
            //             samples_this_block,
            //             samples_per_frame,
            //             samples_into_block,
            //         ) as usize;
            //         if samples_to_do > to_do - samples_written {
            //             println!("INTERSTING!!!!!!!!!!!!!!!!!!!!: {samples_to_do} > {to_do} - {samples_written}");
            //             samples_to_do = to_do - samples_written;
            //         }
            //         let offset_u: usize = offset as usize;
            //         let samples_written_u: usize = samples_written as usize;
            //         let frame_size_u: usize = frame_size as usize;

            //         let frame_offset_start: usize =
            //             offset_u + samples_written_u + decode_pos_samples;
            //         let frame_offset_end: usize =
            //             offset_u + frame_size_u + samples_written_u + decode_pos_samples;
            //         let frame_buffer_size = frame_offset_end - frame_offset_start;
            //         let mut frame: Vec<u8> = vec![0; frame_buffer_size];
            //         frame.copy_from_slice(&self.buffer[frame_offset_start..frame_offset_end]);

            //         println!(
            //             "DONE: {} + {} = {}",
            //             samples_written,
            //             samples_to_do,
            //             samples_written + samples_to_do
            //         );

            //         samples_written += samples_to_do;
            //         current_sample += samples_to_do as i32;
            //         samples_into_block += samples_to_do as i32;
            //     }

            //     decode_pos_samples += to_do;
            // }

            // loop {
            //     if (decode_pos_samples + max_buffer_samples) > length_samples {
            //         to_do = length_samples - decode_pos_samples;
            //     } else {
            //         to_do = max_buffer_samples;
            //     }
            //     if to_do <= 0 {
            //         break;
            //     }

            //     let mut samples_written: usize = 0;
            //     let samples_per_frame: usize =
            //         Parser::decode_get_samples_per_frame(frame_size as i32, channels) as usize;
            //     let samples_this_block: usize = length_samples;

            //     println!("TO_DO: {}, done: {}", to_do, decode_pos_samples);
            //     while samples_written < to_do {
            //         let mut samples_to_do: usize = Parser::decode_get_samples_to_do(
            //         /home/night/Documents/GitHub/sqex_scd_file_parserset as usize;
            //         let samples_written_u: usize = samples_written as usize;
            //         let frame_size_u: usize = frame_size as usize;

            //         let frame_offset_start: usize =
            //             offset_u + samples_written_u + decode_pos_samples;
            //         let frame_offset_end: usize =
            //             offset_u + frame_size_u + samples_written_u + decode_pos_samples;
            //         let frame_buffer_size = frame_offset_end - frame_offset_start;
            //         let mut frame: Vec<u8> = vec![0; frame_buffer_size];
            //         frame.copy_from_slice(&self.buffer[frame_offset_start..frame_offset_end]);

            //         println!(
            //             "DONE: {} + {} = {}",
            //             samples_written,
            //             samples_to_do,
            //             samples_written + samples_to_do
            //         );

            //         samples_written += samples_to_do;
            //         current_sample += samples_to_do;
            //         samples_into_block += samples_to_do;
            //     }
            //     // for index in 0..to_do {
            //     //     println!("F2: {}", index);
            //     // }

            //     decode_pos_samples += to_do;
            // }

            println!("{}", frame_size);

            self.print_column_headers();
            for hex in &self.buffer {
                if self.parse_index > 1000 {
                    break;
                }
                self.parse_cursor = *hex;

                if self.print_column_index < *&self.print_column_count {
                    self.print_row.push(self.parse_cursor);
                    self.print_column_index += 1;
                } else {
                    self.print_row();
                    self.print_row.clear();
                    self.print_column_index = 0;
                    self.print_row_index += self.print_column_count;

                    self.print_row.push(self.parse_cursor);
                    self.print_column_index += 1;
                }

                self.parse_index += 1;
            }
        }

        fn read_hex(&mut self, start: usize, size: usize, color: PrintColor) -> Vec<u8> {
            let mut hex_list: Vec<u8> = Vec::new();
            for index in start..start + size {
                let value = self.buffer[index];
                hex_list.push(value)
            }
            let mark = HexMark {
                start,
                end: start + size,
                size,
                color,
            };
            //let value = String::from_utf8(hex_list.clone()).expect("Failed to parse hex list");
            self.print_marks.push(mark);
            // HexValue {
            //     hex_list,
            //     mark,
            //     value,
            // }
            hex_list
        }

        fn print_row(&self) {
            let row_index = format!("{:06x} ", self.print_row_index);
            self.print_color(&row_index, PrintColor::Red, PrintColor::Default);
            self.print_column();
            self.print_color("\n", PrintColor::Default, PrintColor::Default);
        }

        fn print_column(&self) {
            let length = self.print_row.len();
            for (index, hex_column) in self.print_row.iter().enumerate() {
                let color_pack = self.print_color_picker(index);
                let print_hex = format!("{:02x}", hex_column);

                self.print_color(&print_hex, PrintColor::Default, color_pack.text);
                if index == length / 2 - 1 {
                    self.print_color(" ", PrintColor::Default, color_pack.space);
                }
                if index < length - 1 {
                    self.print_color(" ", PrintColor::Default, color_pack.space);
                }
            }
            // self.print_color("  ", PrintColor::Default, PrintColor::Default);
            // for (index, hex_column) in self.print_row.iter().enumerate() {
            //     let color_pack = self.print_color_picker(index);
            //     let print_hex = format!(
            //         "{}",
            //         if *hex_column > 32 && 127 > *hex_column {
            //             *hex_column as char
            //         } else {
            //             '.'
            //         }
            //     );

            //     self.print_color(&print_hex, PrintColor::Default, color_pack.text);
            //     if index == length / 2 - 1 {
            //         self.print_color(" ", PrintColor::Default, color_pack.space);
            //     }
            //     if index < length - 1 {
            //         self.print_color(" ", PrintColor::Default, color_pack.space);
            //     }
            // }
        }

        fn print_column_headers(&self) {
            self.print_color("Adress ", PrintColor::Red, PrintColor::Default);
            for index in 0..self.print_column_count {
                let print_hex = format!("{:02x}", index);
                self.print_color(&print_hex, PrintColor::Red, PrintColor::Default);
                if index == self.print_column_count / 2 - 1 {
                    self.print_color(" ", PrintColor::Red, PrintColor::Default);
                }
                if index < self.print_column_count - 1 {
                    self.print_color(" ", PrintColor::Red, PrintColor::Default);
                }
            }
            print!("\n\n");
        }

        fn print_color_picker(&self, index: usize) -> PrinterColorPack {
            let mut text_color = PrintColor::Default;
            let mut space_color = PrintColor::Default;
            let relative_index = (self.parse_index - self.print_column_count) + index;
            let mark = self
                .print_marks
                .iter()
                .find(|x| x.start <= relative_index && relative_index <= x.end - 1);
            if let Some(m) = mark {
                text_color = m.color;
                if m.end - 2 >= relative_index {
                    space_color = m.color;
                }
            }
            PrinterColorPack {
                space: space_color,
                text: text_color,
            }
        }

        fn print_color(&self, value: &str, text: PrintColor, background: PrintColor) {
            print!("\x1b[3{};4{}m{}\x1b", text as u8, background as u8, value);
        }

        fn string(&mut self, start: usize, size: usize) -> String {
            String::from_utf8(self.read_hex(start, size, PrintColor::Green)).unwrap()
        }

        fn u8(&mut self, start: usize) -> u8 {
            u8::from_ne_bytes(Parser::buffer_1b(self.read_hex(
                start,
                0x01,
                PrintColor::Yellow,
            )))
        }

        fn i8(&mut self, start: usize) -> i8 {
            i8::from_ne_bytes(Parser::buffer_1b(self.read_hex(
                start,
                0x01,
                PrintColor::Yellow,
            )))
        }

        fn u16(&mut self, start: usize) -> u16 {
            u16::from_ne_bytes(Parser::buffer_2b(self.read_hex(
                start,
                0x02,
                PrintColor::Blue,
            )))
        }

        fn i16(&mut self, start: usize) -> i16 {
            i16::from_ne_bytes(Parser::buffer_2b(self.read_hex(
                start,
                0x02,
                PrintColor::Blue,
            )))
        }

        fn i32(&mut self, start: usize) -> i32 {
            i32::from_ne_bytes(Parser::buffer_4b(self.read_hex(
                start,
                0x04,
                PrintColor::Cyan,
            )))
        }

        fn u32(&mut self, start: usize) -> u32 {
            u32::from_ne_bytes(Parser::buffer_4b(self.read_hex(
                start,
                0x04,
                PrintColor::Cyan,
            )))
        }

        fn u64(&mut self, start: usize) -> u64 {
            u64::from_ne_bytes(Parser::buffer_8b(self.read_hex(
                start,
                0x08,
                PrintColor::Magenta,
            )))
        }

        fn i64(&mut self, start: usize) -> i64 {
            i64::from_ne_bytes(Parser::buffer_8b(self.read_hex(
                start,
                0x08,
                PrintColor::Magenta,
            )))
        }

        fn buffer_8b(buffer: Vec<u8>) -> [u8; 8] {
            let mut static_buffer: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
            Parser::buffer_fill(&mut static_buffer, &buffer);
            static_buffer
        }

        fn buffer_4b(buffer: Vec<u8>) -> [u8; 4] {
            let mut static_buffer: [u8; 4] = [0, 0, 0, 0];
            Parser::buffer_fill(&mut static_buffer, &buffer);
            static_buffer
        }

        fn buffer_2b(buffer: Vec<u8>) -> [u8; 2] {
            let mut static_buffer: [u8; 2] = [0, 0];
            Parser::buffer_fill(&mut static_buffer, &buffer);
            static_buffer
        }

        fn buffer_1b(buffer: Vec<u8>) -> [u8; 1] {
            let mut static_buffer: [u8; 1] = [0];
            Parser::buffer_fill(&mut static_buffer, &buffer);
            static_buffer
        }

        fn buffer_fill(static_buffer: &mut [u8], buffer: &Vec<u8>) {
            let len = static_buffer.len();
            if buffer.len() < len {
                panic!("Buffer is too small: {:?}", buffer);
            }
            for index in 0..len {
                let item = buffer[index];
                static_buffer[index] = item;
            }
        }
    }
}
