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
use iced::widget::{button, column, Column, container, text, Scrollable, row, Row, scrollable, text_input};
use iced::{Alignment, Element, Font, Length, Sandbox, Settings, window, Application, Command, executor, Theme};
use once_cell::sync::Lazy;
use iced::widget::pane_grid::Axis::{Horizontal, Vertical};
use iced::window::{PlatformSpecific, Position};
use game_data_resolver::parser::ffxiv::index1::{Index1, IndexData1};
use game_data_resolver::parser::ffxiv::index_path::IndexPath;
use egui;
use eframe;
use egui::{Color32, FontSelection, ScrollArea, TextEdit, Ui, Widget};
use egui_extras::{Size, StripBuilder};
use env_logger;


static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
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

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    let game_path = "/home/night/.steam/steam/steamapps/common/FINAL FANTASY XIV Online/game/sqpack/";
    let mut file_paths: Vec<PathBuf> = Vec::new();
    get_file_paths(game_path, &mut file_paths);

    let mut multi_text = String::from("");
    let mut search_text = String::from("");

    let mut data1_list: Vec<IndexData1> = Vec::new();

    for file_path in file_paths.clone() {
        let extension = file_path.extension().unwrap();
        if extension == "index" {
            let index_file = fs::read(file_path).unwrap();
            let mut index_file_buffer = Buffer::new(index_file);
            let index = Index1::new(&mut index_file_buffer);
            multi_text = String::new();
            for data in index.data1 {
                data1_list.push(data);
            }
        }
    }

    // for i in 0..100000 {
    //     multi_text.push_str("0123456789\n");
    // }

    // for file_path in file_paths {
    //     let path = file_path.to_str().unwrap();
    //     multi_text.push_str(&format!("{}\n", path));
    // }

    eframe::run_simple_native("No UwU", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            StripBuilder::new(ui)
                .size(Size::exact(20.0))
                //.size(Size::remainder())
                .size(Size::relative(1.0))
                //.size(Size::exact(10.0))
                .vertical(|mut strip| {
                    strip.strip(|builder| {
                        builder.size(Size::remainder()).size(Size::exact(50.0)).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.add(TextEdit::singleline(&mut search_text).desired_width(f32::INFINITY));
                            });
                            strip.cell(|ui| {
                                if ui.button("Search").clicked() {
                                    let index_path = IndexPath::new(&search_text);
                                    if let Ok(path) = index_path {
                                        multi_text = format!("{:#?}", path);
                                        for data1 in &data1_list {
                                            if data1.hash == path.index1_hash {
                                                multi_text.push_str(&format!("\n{}", data1.hash));
                                            }
                                        }
                                    } else if let Err(error) = index_path {
                                        multi_text = error;
                                    }
                                }
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.size(Size::exact(200.0)).size(Size::remainder()).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                //ui.text_edit_multiline(&mut multi_text);
                                // ui.painter().rect_filled(
                                //     ui.available_rect_before_wrap(),
                                //     0.0,
                                //     Color32::BLUE,
                                // );


                                ScrollArea::vertical().auto_shrink([false; 2]).show_viewport(ui, |ui, viewport| {
                                    for file_path in &file_paths {
                                        let extension = file_path.extension().unwrap();
                                        if extension == "index" {
                                            let stem = file_path.file_stem().unwrap().to_str().unwrap();
                                            if ui.button(stem).clicked() {
                                                let index_file = fs::read(file_path).unwrap();
                                                let mut index_file_buffer = Buffer::new(index_file);
                                                let index = Index1::new(&mut index_file_buffer);
                                                multi_text = String::new();
                                                for data in index.data1 {
                                                    multi_text.push_str(&format!("{}\n", data.hash));
                                                }
                                            };
                                        }
                                    }
                                    // let font_id = FontSelection::Default.resolve(ui.style());
                                    // let row_height = ui.fonts(|f| f.row_height(&font_id));
                                    // let height = (ui.available_height() / row_height).floor();
                                    // let text_editor = TextEdit::multiline(&mut multi_text).hint_text("NO").desired_rows(height as usize).desired_width(f32::INFINITY).font(FontSelection::Default);
                                    // ui.add(text_editor);
                                });
                            });
                            strip.cell(|ui| {
                                ScrollArea::vertical().id_source(511).auto_shrink([false; 2]).show_viewport(ui, |ui, viewport| {
                                    // ui.text_edit_multiline(&mut multi_text);
                                    // ui.painter().rect_filled(
                                    //     ui.available_rect_before_wrap(),
                                    //     0.0,
                                    //     Color32::BLUE,
                                    // );
                                    let font_id = FontSelection::Default.resolve(ui.style());
                                    let row_height = ui.fonts(|f| f.row_height(&font_id));
                                    let height = (ui.available_height() / row_height).floor();
                                    let text_editor = TextEdit::multiline(&mut multi_text).hint_text("NO").desired_rows(height as usize).desired_width(f32::INFINITY).font(FontSelection::Default);
                                    ui.add(text_editor);
                                });


                                // ScrollArea::vertical().auto_shrink([false; 2]).show_viewport(ui, |ui, viewport| {
                                //     let font_id = FontSelection::Default.resolve(ui.style());
                                //     let row_height = ui.fonts(|f| f.row_height(&font_id));
                                //     let height = (ui.available_height() / row_height).floor();
                                //     let text_editor = TextEdit::multiline(&mut multi_text).hint_text("NO").desired_rows(height as usize).desired_width(f32::INFINITY).font(FontSelection::Default);
                                //     ui.add(text_editor);
                                // });
                                //
                            });
                        });
                    });
                });
            // ScrollArea::vertical().auto_shrink([true; 2]).show_viewport(ui, |ui, viewport| {
            //     // ui.add(TextEdit::multiline(&mut multi_text).hint_text("NO").desired_width(f32::INFINITY));
            //     // ui.add(TextEdit::multiline(&mut multi_text).hint_text("NO").desired_width(f32::INFINITY));
            //     // ui.horizontal(|ui| {
            //     //     ui.add(TextEdit::multiline(&mut multi_text).hint_text("NO"));
            //     //     ui.add(TextEdit::multiline(&mut multi_text).hint_text("NO"));
            //     // });
            //     //let faded_color = ui
            //     StripBuilder::new(ui)
            //         //.size(Size::exact(50.0))
            //         //.size(Size::remainder())
            //         .size(Size::relative(1.0))
            //         //.size(Size::exact(10.0))
            //         .vertical(|mut strip| {
            //             strip.cell(|ui| {
            //                 ui.text_edit_multiline(&mut multi_text);
            //                 // ui.painter().rect_filled(
            //                 //     ui.available_rect_before_wrap(),
            //                 //     0.0,
            //                 //     Color32::BLUE,
            //                 // );
            //
            //                 //ui.add(TextEdit::multiline(&mut multi_text).desired_rows().hint_text("NO").desired_width(f32::INFINITY));
            //             })
            //         });
            // });
        });
    })
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    //
    // let options = eframe::NativeOptions {
    //     initial_window_size: Some(egui::vec2(320.0, 240.0)),
    //     ..Default::default()
    // };
    //
    // // Our application state:
    // let mut name = "Arthur".to_owned();
    // let mut text = "".to_owned();
    // let mut age = 42;
    //
    // eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
    //     egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
    //         let te = egui::TextEdit::multiline(&mut text).hint_text("NO");
    //         ui.add(te);
    //         ui.text_edit_multiline(&mut text).
    //         ui.button("boom");
    //         ui.horizontal(|ui| {
    //             let name_label = ui.label("Your name: ");
    //             ui.text_edit_singleline(&mut name)
    //                 .labelled_by(name_label.id);
    //         });
    //         ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
    //         if ui.button("Click each year").clicked() {
    //             age += 1;
    //         }
    //         ui.label(format!("Hello '{name}', age {age}"));
    //     });
    // })
}

// fn main() -> iced::Result {
//     Counter::run(Settings::from(Settings {
//         id: None,
//         default_font: None,
//         antialiasing: false,
//         exit_on_close_request: true,
//         text_multithreading: false,
//         try_opengles_first: false,
//         window: window::Settings {
//             size: (1024, 720),
//             position: Position::Centered,
//             min_size: Some((200, 200)),
//             max_size: None,
//             visible: true,
//             resizable: true,
//             decorations: true,
//             transparent: false,
//             always_on_top: false,
//             icon: None,
//             platform_specific: PlatformSpecific,
//         },
//         default_text_size: 20.0,
//         flags: (),
//     }))
// }

#[derive(Default)]
struct Counter {
    text_input_search: String,
    analyzer: String,
    paths: Vec<PathBuf>,
    theme: Theme,
    value: i32,
    index1data: Vec<IndexData1>,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    ButtonSearchPressed,
    Selected(PathBuf),
    TextInputSearchChanged(String),
    TextInputAnalyzerChanged(String),
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

        let mut index1data: Vec<IndexData1> = Vec::new();
        for file_path in file_paths.clone() {
            let file_extension = file_path.extension().unwrap().to_str().unwrap();
            if file_extension == "index" {
                let index_file = fs::read(file_path).unwrap();
                let mut index_file_buffer = Buffer::new(index_file);
                let index = Index1::new(&mut index_file_buffer);
                for data in index.data1 {
                    index1data.push(data);
                }
            }
        }
        let len = index1data.len();

        (
            Counter { index1data, value: 0, theme: Theme::Dark, paths: file_paths, analyzer: String::from(format!("{}", len)), text_input_search: String::from("yes") },
            Command::none()
        )
    }


    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> Command<self::Message> {
        match message {
            Message::ButtonSearchPressed => {
                let index_path = IndexPath::new(&self.text_input_search);
                if let Ok(path) = index_path {
                    self.analyzer = format!("{:#?}", path);
                    for index1datum in &self.index1data {
                        if path.index1_hash == index1datum.hash {
                            self.analyzer.push_str(&format!("\n{}", index1datum.hash));
                        }
                    }
                } else {
                    self.analyzer = String::from("Error parsing the path.");
                }
            }
            Message::TextInputSearchChanged(value) => {
                self.text_input_search = value
            }
            Message::TextInputAnalyzerChanged(value) => {
                self.analyzer = value
            }
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
                        let metadata = parser::ffxiv::index1::Index1::new(&mut buffer);
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
        container(column![
            row![
                text_input("Hello", &self.text_input_search).on_input(Message::TextInputSearchChanged).width(Length::Fill),
                button("Search").on_press(Message::ButtonSearchPressed).width(200)
            ],
            row![
                scrollable((&self.paths).iter().map(|path| -> Element<Message> {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    let file_extension = path.extension().unwrap().to_str().unwrap();
                    Element::from(button(file_name).on_press(Message::Selected(path.clone())).padding(10).width(Length::Fill))
                }).fold(Column::new().spacing(10), |a, b| a.push(b))).width(200),
                scrollable(column![
                    text_input("Nothing!", &self.analyzer).on_input(Message::TextInputAnalyzerChanged),
                ]).width(Length::Fill).height(Length::Fill)
            ]
        ]).width(Length::Fill)
            .height(Length::Fill)
            .into()
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
