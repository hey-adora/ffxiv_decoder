pub mod visualizer {
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

    #[derive(Clone)]
    struct HexMark {
        pub start: usize,
        pub end: usize,
        pub size: usize,
        pub color: PrintColor,
    }

    #[derive(Clone)]
    pub struct PrinterColorPack {
        pub text: PrintColor,
        pub space: PrintColor,
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
}
