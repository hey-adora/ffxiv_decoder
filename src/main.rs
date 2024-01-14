use game_data_resolver::visualizer;
use game_data_resolver::reader;
use game_data_resolver::parser;
use game_data_resolver::decoder;

static FILE_PATH: &str = "./line.scd";

fn main() {
    let mut buffer = reader::Buffer::from_file(FILE_PATH);
    let metadata = parser::sqex_scd::Metadata::new(&mut buffer);
    let decoded = decoder::sqex_scd::decode(metadata, &mut buffer);
    println!("{}", decoded.len());
    for (index, byte) in decoded.iter().enumerate() {
        if index % 16 == 0 {
            print!("\x1b[39;49m\n")
        }
        if *byte > 0 {
            print!("\x1b[31;49m{:02x} ", byte)
        } else {
            print!("\x1b[39;49m{:02x} ", byte)
        }
    }

    std::fs::write("./audio", &decoded).unwrap();

    //visualizer::buffer::print(&buffer);
}

