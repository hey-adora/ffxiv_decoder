pub mod scd_parser {
    //use colored::Colorize;
    use std::fs::File;
    use std::io::{BufReader, Read};

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

    struct HexMark {
        start: usize,
        end: usize,
        color: PrintColor,
    }

    struct HexValue<'h, T> {
        value: T,
        hex_list: &'h [u8],
        mark: HexMark,
    }

    pub struct Parser {
        buffer: Vec<u8>,
        parse_index: usize,
        print_column_count: usize,
        print_row_index: usize,
        print_row: Vec<u8>,
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
            Parser {
                buffer,
                parse_index: 0,
                print_column_count,
                print_row_index: 0,
                print_row: Vec::new(),
            }
        }

        pub fn print(&mut self) {
            let mut loop_index: usize = 0;
            for hex in &self.buffer {
                if loop_index < *&self.print_column_count {
                    self.print_row.push(*hex);
                    loop_index += 1;
                } else {
                    self.print_column();
                    self.print_color("\n", PrintColor::Default);
                    //print!("\n");
                    self.print_row.clear();
                    loop_index = 0;
                }
            }
        }

        fn print_column(&self) {
            let length = self.print_row.len();
            for (index, column) in self.print_row.iter().enumerate() {
                let print_hex = format!("{:02x}", column);
                self.print_color(&print_hex, PrintColor::Red);
                //print!("{}", print_hex.on_red());
                if index < length - 1 {
                    self.print_color(" ", PrintColor::Blue);

                    // print!("{}", " ");
                }
            }
        }

        fn print_color(&self, value: &str, background: PrintColor) {
            print!(
                "\x1b[3{};4{}m{}\x1b",
                PrintColor::Default as u8,
                background as u8,
                value
            );
        }
    }
}
