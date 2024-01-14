use sqex_scd_file_parser::scd_parser::Parser;

use colored::Colorize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str;

static COLUMNS: u8 = 16;
static COLUMNS_GAP: u8 = COLUMNS / 2 - 1;
static COLUMNS_ASCII: u8 = COLUMNS + COLUMNS / 2;
static COLUMNS_ASCII_GAP: u8 = COLUMNS - 1;
static FILE_PATH: &str = "./line.scd";

fn main() {
    let buffer = Parser::read_file(FILE_PATH);
    let mut scd_parser = Parser::new(buffer, 16);
    scd_parser.print();

    // let a = get_hex(&buffer, 0x00, 0x08, PrintHexColor::Green);
    // println!("{}", a.value);

    // let mut column_index: u8 = 0;
    // print!("\x1b[31mAddress\x1b[31m  ");
    // while column_index < COLUMNS {
    //     print_hex(
    //         column_index,
    //         PrintHexColor::Red,
    //         &mut 0,
    //         &mut column_index,
    //         false,
    //     );
    // }
    // print!("\n\n");

    // column_index = 0;
    // let mut row_index: u64 = 0;
    // for value in buffer {
    //     let color: PrintHexColor = match value {
    //         0 => PrintHexColor::Default,
    //         _ => PrintHexColor::Red,
    //     };

    //     print_hex(value, color, &mut row_index, &mut column_index, true);

    //     if row_index > 0x30 {
    //         break;
    //     }
    // }

    //let a = u32::from_be_bytes(vec![0u8, 0u8, 0u8, 9u8]);
    // println!("{}", a);
    //let x = 2;
    //let a = vec![4, 5, 8];
    //let b = &a[1..x];
    //let c = a.slice(1..2);
}

fn get_hex(buffer: &Vec<u8>, start: usize, end: usize, color: PrintHexColor) -> HexValue<&str> {
    let hex_list = &buffer[start..end];
    let value = str::from_utf8(hex_list).expect("Error casting vector to value.");
    let mark = HexMark { start, end, color };
    HexValue {
        value,
        hex_list,
        mark,
    }
}

struct HexMark {
    start: usize,
    end: usize,
    color: PrintHexColor,
}

struct HexValue<'h, T> {
    value: T,
    hex_list: &'h [u8],
    mark: HexMark,
}

enum PrintHexColor {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

fn print_hex(
    hex: u8,
    color: PrintHexColor,
    row_index: &mut u64,
    column_index: &mut u8,
    print_row: bool,
) {
    if *column_index < COLUMNS_ASCII {
        if print_row && *column_index == 0 {
            print!("\x1b[31m{:07x?}\x1b[0m  ", row_index);
        }
        if *column_index < COLUMNS {
            print!("\x1b[3{}m{:02x}\x1b[0m ", get_print_hex_color(color), hex);
        } else {
            print!("\x1b[3{}m{}\x1b[0m ", get_print_hex_color(color), hex);
        }
        if *column_index == COLUMNS_GAP || *column_index == COLUMNS_ASCII_GAP {
            print!(" ");
        }
        *column_index += 1;
    } else {
        print!("\n");
        *column_index = 0;
        *row_index += 1;
    }
}

fn get_print_hex_color(print_hex_color: PrintHexColor) -> u8 {
    match print_hex_color {
        PrintHexColor::Default => 9,
        PrintHexColor::Black => 0,
        PrintHexColor::Red => 1,
        PrintHexColor::Green => 2,
        PrintHexColor::Yellow => 3,
        PrintHexColor::Blue => 4,
        PrintHexColor::Magenta => 5,
        PrintHexColor::Cyan => 6,
        PrintHexColor::White => 7,
    }
}
