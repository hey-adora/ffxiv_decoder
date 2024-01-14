use game_data_resolver::visualizer;
use game_data_resolver::reader;
use game_data_resolver::parser;
use game_data_resolver::decoder;
use std::f32::consts::PI;
use std::i16;
use hound;

static FILE_PATH: &str = "./2.scd";

fn main() {
    let mut buffer = reader::Buffer::from_file(FILE_PATH);
    let metadata = parser::sqex_scd::Metadata::new(&mut buffer);
    if metadata.entry_channels > 2 || metadata.entry_codex != 12 || metadata.entry_wave_format_ex != 7 {
        panic!("Unsupported format");
    }
    let decoded = decoder::sqex_scd::decode(&metadata, &mut buffer);

    let spec = hound::WavSpec {
        channels: metadata.entry_channels as u16,
        sample_rate: metadata.entry_sample_rate as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for index in (0..decoded.len()).step_by(2) {
        writer.write_sample(i16::from_le_bytes([decoded[index], decoded[index + 1]])).unwrap();
    }
    // for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
    //     let sample = (t * 440.0 * 2.0 * PI).sin();
    //     let amplitude = i16::MAX as f32;
    //     writer.write_sample((sample * amplitude) as i16).unwrap();
    // }

    // println!("{}", decoded.len());
    // for (index, byte) in decoded.iter().enumerate() {
    //     if index % 16 == 0 {
    //         print!("\x1b[39;49m\n")
    //     }
    //     if *byte > 0 {
    //         print!("\x1b[31;49m{:02x} ", byte)
    //     } else {
    //         print!("\x1b[39;49m{:02x} ", byte)
    //     }
    // }
    //
    // std::fs::write("./audio", &decoded).unwrap();

    //visualizer::buffer::print(&buffer);
}

