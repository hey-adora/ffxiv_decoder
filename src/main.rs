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
use iced::widget::{button, column, Column, container, text};
use iced::{Alignment, Element, Font, Length, Sandbox, Settings, window, Application};
use iced::widget::pane_grid::Axis::{Horizontal, Vertical};
use iced::window::{PlatformSpecific, Position};


static FILE_PATH: &str = "./2.scd";


fn main() -> iced::Result {
    Counter::run(Settings::from(Settings {
        id: None,
        default_font: None,
        antialiasing: false,
        exit_on_close_request: true,
        text_multithreading: false,
        try_opengles_first: false,
        window: window::Settings {
            size: (200, 200),
            position: Position::Centered,
            min_size: Some((200, 200)),
            max_size: Some((200, 200)),
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
            platform_specific: PlatformSpecific,
        },
        default_text_size: 20.0,
        flags: (),
    }))
}

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1
            }
            Message::DecrementPressed => {
                self.value -= 1
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let children = vec![
            Element::from(button("Increment").on_press(Message::IncrementPressed)),
            Element::from(text(self.value).size(50)),
            Element::from(button("Decrement").on_press(Message::DecrementPressed)),
        ];
        let content = Column::with_children(children)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}


fn decode_zlib() {
    let file = fs::read("./testttt2").unwrap();
    let mut outt: Vec<u8> = Vec::with_capacity(204455);
    let mut y = Decompress::new(false);
    y.decompress_vec(&file, &mut outt, FlushDecompress::Finish).unwrap();
    fs::write("./l23", outt);
}

fn decode_audio() {
    let mut buffer = reader::Buffer::from_file(FILE_PATH);
    let metadata = parser::audio::sqex_scd::Metadata::new(&mut buffer);
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
}
