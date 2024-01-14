pub mod buffer;

use std::collections::HashMap;
use crate::reader::Buffer;

static COLUMN_COUNT: usize = 16;

#[derive(Clone, Copy)]
enum PrintColor {
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

struct ColorLandmark {
    pub start: usize,
    pub size: usize,
    pub end: usize,
    pub color: PrintColor,
}

// #[derive(Clone)]
// struct HexMark {
//     pub start: usize,
//     pub end: usize,
//     pub size: usize,
//     pub color: PrintColor,
// }
//
// #[derive(Clone)]
// pub struct PrinterColorPack {
//     pub text: PrintColor,
//     pub space: PrintColor,
// }

fn print_columns() {
    print_color("Address  ", PrintColor::Red, PrintColor::Default);
    for byte in 0..COLUMN_COUNT {
        print_color(format!("{:02x} ", byte).as_str(), PrintColor::Red, PrintColor::Default);
    }
}

fn print_color_picker(start: usize, size: Option<&usize>, print_color_landmark: &mut ColorLandmark) {
    if let Some(s) = size {
        print_color_landmark.color = match s {
            //4 => PrintColor::Green,
            _ => PrintColor::White
        };
        print_color_landmark.start = start;
        print_color_landmark.end = start + *s;
        print_color_landmark.size = *s;
    } else {
        if start >= print_color_landmark.end {
            print_color_landmark.color = PrintColor::Default;
        }
    }
}

fn print_space(start: usize, print_color_landmark: &ColorLandmark) {
    if start + 1 >= print_color_landmark.end {
        print_color(" ", PrintColor::Default, PrintColor::Default);
    } else {
        print_color(" ", PrintColor::Default, print_color_landmark.color);
    }
}

fn print_ascii(start: usize, buffer: &Buffer) {
    let mut print_color_landmark = ColorLandmark {
        start,
        size: 0,
        end: 0,
        color: PrintColor::Default,
    };
    let start_index = start - COLUMN_COUNT;
    for (index, byte) in buffer.bytes[start_index..start].iter().enumerate() {
        let global_index = index + start_index;
        print_color_picker(global_index, buffer.read_history.get(&global_index), &mut print_color_landmark);
        let letter = if *byte >= 32 && *byte <= 126 { *byte as char } else { '.' };
        print_color(format!("{}", letter).as_str(), PrintColor::Red, print_color_landmark.color);
        print_space(global_index, &print_color_landmark);
    }
}

fn print_color(value: &str, text: PrintColor, background: PrintColor) {
    print!("\x1b[3{};4{}m{}", text as u8, background as u8, value);
}

// fn print_row(&self) {
//     let row_index = format!("{:06x} ", self.print_row_index);
//     self.print_color(&row_index, PrintColor::Red, PrintColor::Default);
//     self.print_column();
//     self.print_color("\n", PrintColor::Default, PrintColor::Default);
// }
//
// fn print_column(&self) {
//     let length = self.print_row.len();
//     for (index, hex_column) in self.print_row.iter().enumerate() {
//         let color_pack = self.print_color_picker(index);
//         let print_hex = format!("{:02x}", hex_column);
//
//         self.print_color(&print_hex, PrintColor::Default, color_pack.text);
//         if index == length / 2 - 1 {
//             self.print_color(" ", PrintColor::Default, color_pack.space);
//         }
//         if index < length - 1 {
//             self.print_color(" ", PrintColor::Default, color_pack.space);
//         }
//     }
// }
//
// fn print_column_headers(&self) {
//     self.print_color("Adress ", PrintColor::Red, PrintColor::Default);
//     for index in 0..self.print_column_count {
//         let print_hex = format!("{:02x}", index);
//         self.print_color(&print_hex, PrintColor::Red, PrintColor::Default);
//         if index == self.print_column_count / 2 - 1 {
//             self.print_color(" ", PrintColor::Red, PrintColor::Default);
//         }
//         if index < self.print_column_count - 1 {
//             self.print_color(" ", PrintColor::Red, PrintColor::Default);
//         }
//     }
//     print!("\n\n");
// }
//
// fn print_color_picker(&self, index: usize) -> PrinterColorPack {
//     let mut text_color = PrintColor::Default;
//     let mut space_color = PrintColor::Default;
//     let relative_index = (self.parse_index - self.print_column_count) + index;
//     let mark = self
//         .print_marks
//         .iter()
//         .find(|x| x.start <= relative_index && relative_index <= x.end - 1);
//     if let Some(m) = mark {
//         text_color = m.color;
//         if m.end - 2 >= relative_index {
//             space_color = m.color;
//         }
//     }
//     PrinterColorPack {
//         space: space_color,
//         text: text_color,
//     }
// }
//
// fn print_color(&self, value: &str, text: PrintColor, background: PrintColor) {
//     print!("\x1b[3{};4{}m{}\x1b", text as u8, background as u8, value);
// }