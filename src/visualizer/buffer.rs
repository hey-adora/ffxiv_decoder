use std::fmt::format;
use super::*;


pub fn print(buffer: &Buffer) {
    print_columns();
    print!("\n\n000000   ");
    let mut print_color_landmark = ColorLandmark {
        start: 0,
        size: 0,
        end: 0,
        color: PrintColor::Default,
    };
    for (index, byte) in buffer.bytes.iter().enumerate() {
        if index != 0 && index % COLUMN_COUNT == 0 {
            print_color(" ", PrintColor::Default, PrintColor::Default);
            print_ascii(index, &buffer); 
            print_color(format!("\n{:06x}   ", index / COLUMN_COUNT).as_str(), PrintColor::Red, PrintColor::Default);
        }
        print_color_picker(index, buffer.read_history.get(&index), &mut print_color_landmark);
        print_color(format!("{:02x}", byte).as_str(), PrintColor::Default, print_color_landmark.color);
        print_space(index, &print_color_landmark);
    }
}

