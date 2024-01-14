use std::fs::File;
use std::io::{BufReader, Read};
use colored::Colorize;

fn main() {
    const COLUMNS: u8 = 16;
    const COLUMNS_GAP: u8 = COLUMNS / 2 - 1;
    const FILE_PATH: &str = "./line.scd";

    let file = File::open(FILE_PATH).expect("Failed to open file.");
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).expect("Failed to read file.");


    print!("\x1b[31mAddress\x1b[31m  ");
    for column in 0..COLUMNS {
        print!("\x1b[31m{:02x}\x1b[31m ", column);
        if column == COLUMNS_GAP {
            print!(" ");
        }
    }
    print!("\n\n");

    let mut column_index: u8 = 0;
    let mut row_index: u64 = 0;
    for value in buffer {
        if column_index < COLUMNS {
            if column_index == 0 {
                print!("\x1b[31m{:07x?}\x1b[0m  ", row_index);
            }
            print!("\x1b[3{}m{:02x}\x1b[0m ", if value > 0 {2} else {9} ,value);
            if column_index == COLUMNS_GAP {
                print!(" ");
            }
            column_index += 1;
        } else {
            print!("\n");
            column_index = 0;
            row_index += 1;
        }
    }
}
