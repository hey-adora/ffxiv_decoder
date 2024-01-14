use std::arch::x86_64::_mm_crc32_u32;
use game_data_resolver::visualizer;
use game_data_resolver::reader;
use game_data_resolver::parser;
use game_data_resolver::decoder;
use game_data_resolver::parser::ffxiv;
use std::f32::consts::PI;
use std::{fs, i16};
use std::io::Read;
use std::path::{Path, PathBuf};
use hound;
use crc32fast::Hasher;
use std::io::prelude::*;
use flate2::{Compression, FlushDecompress};
use flate2::Decompress;
use flate2::write::ZlibEncoder;
use flate2::read::GzDecoder;
use flate2::read::ZlibDecoder;
use game_data_resolver::reader::Buffer;
use iced::widget::{button, column, Column, container, text, Scrollable, row, Row, scrollable};
use iced::{Alignment, Element, Font, Length, Sandbox, Settings, window, Application, Command, executor, Theme};
use iced::widget::pane_grid::Axis::{Horizontal, Vertical};
use iced::window::{PlatformSpecific, Position};


static FILE_PATH: &str = "./2.scd";

enum Group {
    Index,
    Data,
}

struct File {
    group: Group,
    path: PathBuf,
}

fn get_file_paths(input_path: &str, output: &mut Vec<PathBuf>) {
    let verify = Path::new(input_path);

    let flag = verify.is_dir();
    if !flag {
        panic!("Path is not a directory: {}", input_path)
    }

    let flag = verify.exists();
    if !flag {
        panic!("Path doesn't not exist: {}", input_path)
    }

    let paths = fs::read_dir(input_path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            output.push(path)
        } else {
            get_file_paths(path.to_str().unwrap(), output);
        }
    }
}


fn main() -> iced::Result {
    Counter::run(Settings::from(Settings {
        id: None,
        default_font: None,
        antialiasing: false,
        exit_on_close_request: true,
        text_multithreading: false,
        try_opengles_first: false,
        window: window::Settings {
            size: (1024, 720),
            position: Position::Centered,
            min_size: Some((200, 200)),
            max_size: None,
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

#[derive(Default)]
struct Counter {
    analyzer: String,
    paths: Vec<PathBuf>,
    theme: Theme,
    value: i32,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    Selected(PathBuf),
}

impl Application for Counter {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Counter, Command<self::Message>) {
        let game_path = "/home/night/.steam/steam/steamapps/common/FINAL FANTASY XIV Online/game/sqpack/";
        let mut file_paths: Vec<PathBuf> = Vec::new();
        get_file_paths(game_path, &mut file_paths);

        (
            Counter { value: 0, theme: Theme::Dark, paths: file_paths, analyzer: String::from("Hello") },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> Command<self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1
            }
            Message::DecrementPressed => {
                self.value -= 1
            }
            Message::Selected(path) => {
                if path.is_file() && path.extension().unwrap().to_str().unwrap() == "index" {
                    let path_str = path.to_str().unwrap();
                    let file = fs::read(path_str);
                    if let Ok(file) = file {
                        let mut buffer = Buffer::new(file);
                        let metadata = parser::ffxiv::index::Metadata::new(&mut buffer);
                        let mut analizer = String::new();
                        for data in metadata.data1 {
                            analizer.push_str(format!("{}\n", data.hash).as_str())
                        }
                        self.analyzer = analizer
                    } else if let Err(err) = file {
                        self.analyzer = String::from(format!("Error reading file: {} \n{}", path_str, err.to_string()))
                    }
                } else {
                    self.analyzer = String::from(format!("Invalid file: {}", path.to_str().unwrap()))
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut rows: Row<Message> = Row::new();
        // for index in 0..100 {
        //     let column: Column<Message> = Column::new();
        //     let text = Element::from(text(index));
        //     let column = column.push(text);
        //
        //     rows = rows.push(column);
        // }

        let mut column: Column<Message> = Column::new();
        for path in &self.paths {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_extension = path.extension().unwrap().to_str().unwrap();
            if file_extension == "index" {}
            let btn = Element::from(button(file_name).on_press(Message::Selected(path.clone())).padding(10).width(Length::Fill));
            column = column.push(btn);
        }

        let scroll_container = scrollable(column.spacing(10))
            .width(200);

        rows = rows.push(scroll_container);

        let mut column: Column<Message> = Column::new();
        let text = Element::from(text(self.analyzer.as_str()));
        column = column.push(text);

        let scroll_container = scrollable(column)
            .width(Length::Fill);

        rows = rows.push(scroll_container);

        container(rows)
            .width(Length::Fill)
            .height(Length::Fill)
            // .center_x()
            //   .center_y()
            .into()

        // scrollable(rows)
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .into()
    }

    fn theme(&self) -> Theme { self.theme.clone() }
}


fn decode_zlib() {
    let file = fs::read("./0").unwrap();
    let mut outt: Vec<u8> = Vec::with_capacity(204455);
    let mut y = Decompress::new(false);
    y.decompress_vec(&file, &mut outt, FlushDecompress::Finish).unwrap();
    fs::write("./l0", outt);
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
