use game_data_resolver::visualizer;
use game_data_resolver::reader;
use game_data_resolver::parser;
use game_data_resolver::decoder;

static FILE_PATH: &str = "./line.scd";

fn main() {
    let mut buffer = reader::Buffer::from_file(FILE_PATH);
    let metadata = parser::sqex_scd::Metadata::new(&mut buffer);
    let decoded = decoder::sqex_scd::decode(metadata, &mut buffer);
    //println!("{}", 17 / 16);
    visualizer::buffer::print(&buffer);
}

