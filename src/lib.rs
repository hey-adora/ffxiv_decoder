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

            let buffer_size_kb = 1;
            let buffer_size = 1024 * buffer_size_kb;

            let samples_num =
                Parser::msadpcm_bytes_to_samples(stream_size, frame_size as i32, channels);

            let samples_to_do: i32 = 0;
            let to_do: i32 = 0;
            let decode_pos_samples: i32 = 0;
            let max_buffer_samples = buffer_size / (channels * 2);
            let length_samples: i32 = samples_num;
            let play_forever: i32 = 0;

            println!("{}", max_buffer_samples);

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
