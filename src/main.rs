use std::arch::x86_64::_mm_crc32_u32;
use game_data_resolver::visualizer;
use game_data_resolver::reader;
use game_data_resolver::parser;
use game_data_resolver::decoder;
use game_data_resolver::game_path::GamePath;
use std::f32::consts::PI;
use std::{fs, i16};
use std::io::Read;
use std::path::Path;


use hound;
use crc32fast::Hasher;
use std::io::prelude::*;
use flate2::{Compression, FlushDecompress};
use flate2::Decompress;
use flate2::write::ZlibEncoder;
use flate2::read::GzDecoder;
use flate2::read::ZlibDecoder;
use game_data_resolver::reader::Buffer;


static FILE_PATH: &str = "./2.scd";


fn main() {
    //Buffer::new();

    // let o = Vec::new();
    // let mut e = ZlibEncoder::new(o, Compression::default());
    //let file1 = fs::read("./root.exl").unwrap();
    // e.write_all(file1.as_slice());
    // //e.write_all(b"bar");
    // let compressed_bytes = e.finish().unwrap();
    // fs::write("./l", compressed_bytes);
    //

    //let mut header = vec![120u8, 0x9C];
    //header.append(&mut file);
    // file[0] = 0x78;
    // file[0] = 0x01;
    //let y = "...".as_bytes();

    let file = fs::read("./testttt2").unwrap();
    let mut outt: Vec<u8> = Vec::with_capacity(204455);
    let mut y = Decompress::new(false);
    y.decompress_vec(&file, &mut outt, FlushDecompress::Finish).unwrap();
    fs::write("./l23", outt);

    //let mut d = ZlibDecoder::new(file.as_slice());

    let mut s = String::new();

    //
    //d.read_to_end(&mut outt).unwrap();
    println!("{}", s);

    // let game_path = "/home/night/.steam/steam/steamapps/common/FINAL FANTASY XIV Online/game/sqpack/";
    // let game_path_exists = Path::new(game_path).exists();
    // if !game_path_exists {
    //     panic!("Game path doesn't not exist.")
    // }
    //
    // let parsed_path = GamePath::new("exd/root.exl");
    // println!("{:?}", parsed_path)

    //let game_dirs2: Vec<String> = fs::read_dir(game_path).expect("eee").map(|d| d.unwrap().path().to_str().unwrap().to_owned()).collect();
    // let game_dirs: Vec<String> = fs::read_dir(game_path).unwrap().map(|d| d.unwrap().path().to_str().unwrap().to_owned()).collect();
    // for game_dir in game_dirs {
    //     println!("{}", game_dir);
    // }

    // let game_dirs= fs::read_dir(game_path).unwrap();
    // for game_dir in game_dirs {
    //     println!("{}", game_dir);
    // }


    // let game_dir_components: Vec<&str> = game_dir.components().map(|c| c.as_os_str().to_str().unwrap()).collect();
    // let game_dir_str = game_dir_components.join("/");


    // let mut buffer = reader::Buffer::from_file(FILE_PATH);
    // let metadata = parser::sqex_scd::Metadata::new(&mut buffer);
    // if metadata.entry_channels > 2 || metadata.entry_codex != 12 || metadata.entry_wave_format_ex != 7 {
    //     panic!("Unsupported format");
    // }
    // let decoded = decoder::sqex_scd::decode(&metadata, &mut buffer);
    //
    // let spec = hound::WavSpec {
    //     channels: metadata.entry_channels as u16,
    //     sample_rate: metadata.entry_sample_rate as u32,
    //     bits_per_sample: 16,
    //     sample_format: hound::SampleFormat::Int,
    // };
    // let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    // for index in (0..decoded.len()).step_by(2) {
    //     writer.write_sample(i16::from_le_bytes([decoded[index], decoded[index + 1]])).unwrap();
    // }
    //parse_path("exd/root.exl");
}


fn parse_path(path: &str) {
    //path.split('/')
}
