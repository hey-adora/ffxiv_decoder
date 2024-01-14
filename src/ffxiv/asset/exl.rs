use std::str::FromStr;

pub struct EXL {
    pub lines: Vec<(String, i32)>
}

impl EXL {
    pub fn from_vec(data: Vec<u8>) -> EXL {
        let mut lines: Vec<(String, i32)> = Vec::new();
        let mut line: String = String::new();
        for byte in data {
            if byte == b'\n' || byte == b'\r' {
                if line.len() > 3 {
                    let line_parts: Vec<&str> = line.split(',').collect();
                    lines.push((line_parts[0].to_owned(), i32::from_str(line_parts[1]).unwrap()));
                    line = String::new();
                }
            } else {
                line.push(byte as char)
            }
        }
        EXL {
            lines
        }
    }
}