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

    impl Parser {
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
            let gg = self.read_hex(0x00, 0x08, PrintColor::Green);
            println!("{}", gg.value);
            self.print_column_headers();
            for hex in &self.buffer {
                self.parse_cursor = *hex;

                if self.print_column_index < *&self.print_column_count {
                    self.print_row.push(self.parse_cursor);
                    self.print_column_index += 1;
                } else {
                    self.print_row();
                    self.print_row.clear();
                    self.print_column_index = 0;
                }

                self.parse_index += 1;
            }
        }

        fn read_hex(&mut self, start: usize, size: usize, color: PrintColor) -> HexValue<String> {
            let mut hex_list: Vec<u8> = Vec::new();
            for index in (start..size) {
                let value = self.buffer[index];
                hex_list.push(value)
            }
            let mark = HexMark {
                start,
                end: start + size,
                size,
                color,
            };
            let value = String::from_utf8(hex_list.clone()).expect("Failed to parse hex list");
            self.print_marks.push(mark.clone());
            HexValue {
                hex_list,
                mark,
                value,
            }
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
            self.print_color("  ", PrintColor::Default, PrintColor::Default);
            for (index, hex_column) in self.print_row.iter().enumerate() {
                let color_pack = self.print_color_picker(index);
                let print_hex = format!(
                    "{}",
                    if *hex_column > 32 && 127 > *hex_column {
                        *hex_column as char
                    } else {
                        '.'
                    }
                );

                self.print_color(&print_hex, PrintColor::Default, color_pack.text);
                if index == length / 2 - 1 {
                    self.print_color(" ", PrintColor::Default, color_pack.space);
                }
                if index < length - 1 {
                    self.print_color(" ", PrintColor::Default, color_pack.space);
                }
            }
        }

        fn print_column_headers(&self) {
            self.print_color("Adress ", PrintColor::Red, PrintColor::Default);
            for index in (0..self.print_column_count) {
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
                .find(|x| x.start >= relative_index - index && relative_index <= x.end);
            if let Some(m) = mark {
                text_color = m.color;
                if m.end != relative_index {
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
    }
}
