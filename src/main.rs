use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::ops::Deref;
use std::path::PathBuf;
use game_data_resolver::ffxiv::{FFXIV, test};

fn main() {
    //let path = PathBuf::from("./test/test2/test.txt");
    //println!("{}", path.parent().unwrap().to_str().unwrap());
    //create_dir_all("./test/test2/test.txt").unwrap();

    // let mut file = File::create("./dude.txt").unwrap();
    // for i in 0..100u8 {
    //     file.write(&[i]).unwrap();
    // }

    test("/home/night/.steam/steam/steamapps/common/FINAL FANTASY XIV Online/game/sqpack/", "music/ffxiv/BGM_PvP_Battle_02.scd");

    //test("/home/night/.steam/steam/steamapps/common/FINAL FANTASY XIV Online/game/sqpack/", "sound/vfx/monster/se_vfx_monster_reming_revolvekick.scd");
    //test2("/home/night/.steam/steam/steamapps/common/FINAL FANTASY XIV Online/game/sqpack/");

    // let mut o = AtomicUsize::new(1);
    // let mut spinlock = Arc::new(o);
    // let spinlock_clone = Arc::clone(&spinlock);
    // let thread = thread::spawn(move|| {
    //     spinlock_clone.store(0, Ordering::SeqCst);
    // });
    //
    // // Wait for the other thread to release the lock
    // while spinlock.load(Ordering::SeqCst) != 0 {
    //     hint::spin_loop();
    // }
    //
    // let mut spinlock_clone_clone = Arc::clone(&spinlock);
    // let gga = &spinlock_clone_clone.deref().clone().get_mut();
    // println!("{}", gga);
    //
    // let mut some_var = AtomicUsize::new(10);
    // let a = some_var.get_mut();
    // println!("{}", a);
    // //assert_eq!(*some_var.get_mut(), 10);
    // *some_var.get_mut() = 5;
    // let a = some_var.get_mut();
    // println!("{}", a);
    // //assert_eq!(some_var.load(Ordering::SeqCst), 5);


    //let ffxiv = FFXIV::new("/home/night/.steam/steam/steamapps/common/FINAL FANTASY XIV Online/game/sqpack/");
    //println!("damm");
}

// use std::arch::x86_64::_mm_crc32_u32;
// use game_data_resolver::visualizer;
// use game_data_resolver::reader;
// use game_data_resolver::parser;
// use game_data_resolver::decoder;
// use game_data_resolver::parser::ffxiv;
// use std::f32::consts::PI;
// use std::{fs, i16, str, string};
// use std::collections::HashMap;
// use std::fs::File;
// use std::io::{BufWriter, Read, stdout};
// use std::path::{Path, PathBuf};
// use hound;
// use crc32fast::Hasher;
// use std::io::prelude::*;
// use flate2::{Compression, FlushDecompress};
// use flate2::Decompress;
// use flate2::write::ZlibEncoder;
// use flate2::read::GzDecoder;
// use flate2::read::ZlibDecoder;
// use game_data_resolver::reader::Buffer;
// use iced::widget::{button, column, Column, container, text, Scrollable, row, Row, scrollable, text_input};
// use iced::{Alignment, Element, Font, Length, Sandbox, Settings, window, Application, Command, executor, Theme};
// use once_cell::sync::Lazy;
// use iced::widget::pane_grid::Axis::{Horizontal, Vertical};
// use iced::window::{PlatformSpecific, Position};
// use game_data_resolver::parser::ffxiv::index::{Index, Index1Data1, Index2Data1};
// use game_data_resolver::parser::ffxiv::index_path::IndexPath;
// use egui;
// use eframe;
// use egui::{Align2, Color32, FontSelection, pos2, ScrollArea, Sense, TextEdit, TextStyle, Ui, Widget};
// use std::collections::HashSet;
// use egui_extras::{Size, StripBuilder};
// use env_logger;
//
//
// static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
// static FILE_PATH: &str = "./2.scd";
// //
// // enum Group {
// //     Index,
// //     Data,
// // }
// //
// // struct File {
// //     group: Group,
// //     path: PathBuf,
// // }
//
// fn get_file_paths(input_path: &str, output: &mut Vec<PathBuf>) {
//     let verify = Path::new(input_path);
//
//     let flag = verify.is_dir();
//     if !flag {
//         panic!("Path is not a directory: {}", input_path)
//     }
//
//     let flag = verify.exists();
//     if !flag {
//         panic!("Path doesn't not exist: {}", input_path)
//     }
//
//     let paths = fs::read_dir(input_path).unwrap();
//     for path in paths {
//         let path = path.unwrap().path();
//         if path.is_file() {
//             output.push(path)
//         } else {
//             get_file_paths(path.to_str().unwrap(), output);
//         }
//     }
// }
//
// fn main() {
//     let game_path = "/home/night/.steam/steam/steamapps/common/FINAL FANTASY XIV Online/game/sqpack/";
//     let mut file_paths: Vec<PathBuf> = Vec::new();
//     get_file_paths(game_path, &mut file_paths);
//
//     let mut index1_data1_list: HashMap<u64, Index1Data1> = HashMap::new();
//     let mut index2_data1_list: Vec<Index2Data1> = Vec::new();
//
//     for file_path in file_paths.clone() {
//         let extension = file_path.extension().unwrap();
//         if extension == "index" {
//             let index_file = fs::read(&file_path).unwrap();
//             let mut index_file_buffer = Buffer::new(index_file);
//             let index = Index::from_index1(&mut index_file_buffer);
//             for data in index.data1 {
//                 index1_data1_list.insert(data.hash, data);
//             }
//         } else if extension == "index2" {
//             let index_file = fs::read(&file_path).unwrap();
//             let mut index_file_buffer = Buffer::new(index_file);
//             let index = Index::from_index2(&mut index_file_buffer);
//             for data in index.data1 {
//                 index2_data1_list.push(data);
//             }
//         }
//     }
//
//     combine_paths();
//
//     let paths = file_to_lines_hashset("./all_paths.txt");
//     let mut parsed_paths: HashMap<u64, IndexPath> = HashMap::new();
//
//     // let stdout = stdout();
//     // let lock = stdout.lock();
//     // let mut buf_writer = BufWriter::new(lock);
//
//
//     let max_index = paths.len() as f32;
//     let mut error_log = File::create("error_log.txt").unwrap();
//     for (index, path) in paths.iter().enumerate() {
//         let parsed_path = IndexPath::new(&path);
//         if let Ok(path) = parsed_path {
//             // if path.file_extension == "scd" {
//             //     println!("{}", {&path.full_path});
//             // }
//             parsed_paths.insert(path.index1_hash, path);
//         } else if let Err(e) = parsed_path {
//             error_log.write(format!("{} - {}\n", path, e).as_bytes()).unwrap();
//         }
//         if index % 10000 == 0 {
//             let done: f32 = (index as f32 / max_index) * 100.0;
//             println!("Parsing path: {}%.\n", done);
//         }
//     }
//
//     let mut output = String::new();
//     let max_index = index1_data1_list.len() as f32;
//     for (index, (hash, indexdata)) in index1_data1_list.iter().enumerate() {
//         let parsed_path = parsed_paths.get(&hash);
//         if let Some(path) = parsed_path {
//             if path.file_extension == "scd" {
//                 println!("{}", {&path.full_path});
//             }
//             let line = format!("{} {}\n", path.index1_hash, path.full_path);
//             output.push_str(&line);
//         } else {
//             let line = format!("{}\n", hash);
//             output.push_str(&line)
//         }
//         if index % 10000 == 0 {
//             let done: f32 = (index as f32 / max_index) * 100.0;
//             println!("Writing path: {}%.\n", done);
//         }
//     }
//
//     fs::write("has.txt", output).unwrap();
// }
//
// fn combine_paths() {
//     let mut paths: HashSet<String> = HashSet::with_capacity(3000000);
//     add_paths2("./CurrentPathList", &mut paths);
//     add_paths2("./PathList", &mut paths);
//     let mut file = File::create("all_paths.txt").unwrap();
//     for path in paths {
//         file.write(path.as_bytes());
//     }
// }
//
// fn file_to_lines_vec(file_name: &str) -> Vec<String> {
//     let mut paths: Vec<String> = Vec::new();
//
//     let file = fs::read(file_name).unwrap();
//
//     let mut previous_index = 0;
//     for (index, byte) in (&file).iter().enumerate() {
//         if *byte == b'\n' && index > (previous_index + 1) {
//             let path_slice = &file[previous_index..index];
//             let path = String::from_utf8(path_slice.to_owned()).unwrap();
//             paths.push(path);
//             previous_index = index + 1;
//         }
//     }
//
//     paths
// }
//
// fn file_to_lines_hashset(file_name: &str) -> HashSet<String> {
//     let mut paths: HashSet<String> = HashSet::new();
//
//     let file = fs::read(file_name).unwrap();
//
//     let mut previous_index = 0;
//     for (index, byte) in (&file).iter().enumerate() {
//         if *byte == b'\n' && index > (previous_index + 1) {
//             let path_slice = &file[previous_index..index];
//             let path = String::from_utf8(path_slice.to_owned()).unwrap();
//             paths.insert(path);
//             previous_index = index + 1;
//         }
//     }
//
//     paths
// }
//
//
// fn add_paths(file_name: &str, paths: &mut Vec<String>) {
//     let paths_file = fs::read(file_name).unwrap();
//     let mut previous_index = 0;
//     for (index, byte) in (&paths_file).iter().enumerate() {
//         if *byte == b'\n' && index > (previous_index + 1) {
//             let path_slice = &paths_file[previous_index..index];
//             let path = String::from_utf8(path_slice.to_owned()).unwrap();
//             let position = paths.iter().position(|p| *p == path);
//             if let None = position {
//                 paths.push(path);
//                 previous_index = index + 1;
//             }
//         }
//     }
// }
//
// fn add_paths2(file_name: &str, paths: &mut HashSet<String>) {
//     let paths_file = fs::read(file_name).unwrap();
//     let mut previous_index = 0;
//     for (index, byte) in (&paths_file).iter().enumerate() {
//         if *byte == b'\n' && index > previous_index {
//             let path_slice = &paths_file[previous_index..index];
//             let path = String::from_utf8(path_slice.to_owned()).unwrap();
//             paths.insert(path);
//             previous_index = index;
//         }
//     }
// }
//
//
// // fn main() -> Result<(), eframe::Error> {
// //     let options = eframe::NativeOptions {
// //         initial_window_size: Some(egui::vec2(1280.0, 720.0)),
// //         ..Default::default()
// //     };
// //
// //     let game_path = "/home/night/.steam/steam/steamapps/common/FINAL FANTASY XIV Online/game/sqpack/";
// //     let mut file_paths: Vec<PathBuf> = Vec::new();
// //     get_file_paths(game_path, &mut file_paths);
// //
// //     let mut multi_text = String::from("");
// //     let mut search_text = String::from("");
// //
// //     let mut picked_list1: Vec<Index1Data1> = Vec::new();
// //     let mut picked_list2: Vec<Index2Data1> = Vec::new();
// //
// //     let mut index1_data1_list: Vec<Index1Data1> = Vec::new();
// //     let mut index2_data1_list: Vec<Index2Data1> = Vec::new();
// //
// //     for file_path in file_paths.clone() {
// //         let extension = file_path.extension().unwrap();
// //         if extension == "index" {
// //             let index_file = fs::read(&file_path).unwrap();
// //             let mut index_file_buffer = Buffer::new(index_file);
// //             let index = Index::from_index1(&mut index_file_buffer);
// //             multi_text = String::new();
// //             for data in index.data1 {
// //                 index1_data1_list.push(data);
// //             }
// //         } else if extension == "index2" {
// //             let index_file = fs::read(&file_path).unwrap();
// //             let mut index_file_buffer = Buffer::new(index_file);
// //             let index = Index::from_index2(&mut index_file_buffer);
// //             multi_text = String::new();
// //             for data in index.data1 {
// //                 index2_data1_list.push(data);
// //             }
// //         }
// //     }
// //
// //     // for i in 0..100000 {
// //     //     multi_text.push_str("0123456789\n");
// //     // }
// //
// //     // for file_path in file_paths {
// //     //     let path = file_path.to_str().unwrap();
// //     //     multi_text.push_str(&format!("{}\n", path));
// //     // }
// //
// //     eframe::run_simple_native("No UwU", options, move |ctx, _frame| {
// //         egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
// //             StripBuilder::new(ui)
// //                 .size(Size::exact(20.0))
// //                 .size(Size::remainder())
// //                 //.size(Size::relative(1.0))
// //                 //.size(Size::exact(10.0))
// //                 .vertical(|mut strip| {
// //                     strip.strip(|builder| {
// //                         builder.size(Size::remainder()).size(Size::exact(50.0)).horizontal(|mut strip| {
// //                             strip.cell(|ui| {
// //                                 ui.add(TextEdit::singleline(&mut search_text).desired_width(f32::INFINITY));
// //                             });
// //                             strip.cell(|ui| {
// //                                 if ui.button("Search").clicked() {
// //                                     let index_path = IndexPath::new(&search_text);
// //                                     if let Ok(path) = index_path {
// //                                         multi_text = format!("{:#?}", path);
// //                                         for data1 in &index1_data1_list {
// //                                             if data1.hash == path.index1_hash {
// //                                                 multi_text.push_str(&format!("\n{}", data1.hash));
// //                                             }
// //                                         }
// //                                         for data1 in &index2_data1_list {
// //                                             if data1.hash == path.index2_hash {
// //                                                 multi_text.push_str(&format!("\n{}", data1.hash));
// //                                             }
// //                                         }
// //                                     } else if let Err(error) = index_path {
// //                                         multi_text = error;
// //                                     }
// //                                 }
// //                             });
// //                         });
// //                     });
// //                     strip.strip(|builder| {
// //                         builder.size(Size::exact(200.0)).size(Size::remainder()).horizontal(|mut strip| {
// //                             strip.cell(|ui| {
// //                                 //ui.text_edit_multiline(&mut multi_text);
// //                                 // ui.painter().rect_filled(
// //                                 //     ui.available_rect_before_wrap(),
// //                                 //     0.0,
// //                                 //     Color32::BLUE,
// //                                 // );
// //
// //
// //                                 ScrollArea::vertical().auto_shrink([false; 2]).show_viewport(ui, |ui, viewport| {
// //                                     for file_path in &file_paths {
// //                                         let extension = file_path.extension().unwrap();
// //                                         if extension == "index" {
// //                                             let stem = file_path.file_name().unwrap().to_str().unwrap();
// //                                             if ui.button(stem).clicked() {
// //                                                 let index_file = fs::read(file_path).unwrap();
// //                                                 let mut index_file_buffer = Buffer::new(index_file);
// //                                                 let index = Index::from_index1(&mut index_file_buffer);
// //                                                 picked_list1 = Vec::new();
// //                                                 //multi_text = String::new();
// //                                                 for data in index.data1 {
// //                                                     picked_list1.push(data);
// //                                                     //multi_text.push_str(&format!("{}\n", data.hash));
// //                                                 }
// //                                             };
// //                                         } else if extension == "index2" {
// //                                             let stem = file_path.file_name().unwrap().to_str().unwrap();
// //                                             if ui.button(stem).clicked() {
// //                                                 let index_file = fs::read(file_path).unwrap();
// //                                                 let mut index_file_buffer = Buffer::new(index_file);
// //                                                 let index = Index::from_index2(&mut index_file_buffer);
// //                                                 picked_list2 = Vec::new();
// //                                                 //multi_text = String::new();
// //                                                 for data in index.data1 {
// //                                                     picked_list2.push(data);
// //                                                     //multi_text.push_str(&format!("{}\n", data.hash));
// //                                                 }
// //                                             };
// //                                         }
// //                                     }
// //                                     // let font_id = FontSelection::Default.resolve(ui.style());
// //                                     // let row_height = ui.fonts(|f| f.row_height(&font_id));
// //                                     // let height = (ui.available_height() / row_height).floor();
// //                                     // let text_editor = TextEdit::multiline(&mut multi_text).hint_text("NO").desired_rows(height as usize).desired_width(f32::INFINITY).font(FontSelection::Default);
// //                                     // ui.add(text_editor);
// //                                 });
// //                             });
// //                             strip.cell(|ui| {
// //                                 ScrollArea::vertical().id_source(511).auto_shrink([false; 2]).show_viewport(ui, |ui, viewport| {
// //                                     // ui.text_edit_multiline(&mut multi_text);
// //                                     // ui.painter().rect_filled(
// //                                     //     ui.available_rect_before_wrap(),
// //                                     //     0.0,
// //                                     //     Color32::BLUE,
// //                                     // );
// //
// //                                     // let font_id = FontSelection::Default.resolve(ui.style());
// //                                     // let row_height = ui.fonts(|f| f.row_height(&font_id));
// //                                     // let height = (ui.available_height() / row_height).floor();
// //                                     // let text_editor = TextEdit::multiline(&mut multi_text).hint_text("NO").desired_width(f32::INFINITY);
// //                                     // ui.add(text_editor);
// //                                     //
// //
// //                                     // let font_id = FontSelection::Default.resolve(ui.style());
// //                                     // let row_height = ui.fonts(|f| f.row_height(&font_id));
// //                                     // let height = (ui.available_height() / row_height).floor();
// //                                     // let text_editor = TextEdit::multiline(&mut multi_text).hint_text("NO").desired_rows(height as usize).desired_width(f32::INFINITY).font(FontSelection::Default);
// //                                     // ui.add(text_editor);
// //
// //
// //                                     // let font_id = TextStyle::Body.resolve(ui.style());
// //                                     // let r = ui.min_rect();
// //                                     // let x = r.left();
// //                                     // let y = r.top();
// //                                     // let rect = ui.painter().text(
// //                                     //     pos2(x, y),
// //                                     //     Align2::LEFT_TOP,
// //                                     //     &multi_text,
// //                                     //     font_id,
// //                                     //     ui.visuals().text_color(),
// //                                     // );
// //                                     // ui.allocate_rect(rect, Sense::hover());
// //
// //                                     for data in &picked_list1 {
// //                                         ui.button(data.hash.to_string());
// //                                     }
// //                                     for data in &picked_list2 {
// //                                         ui.button(data.hash.to_string());
// //                                     }
// //                                 });
// //
// //
// //                                 // ScrollArea::vertical().auto_shrink([false; 2]).show_viewport(ui, |ui, viewport| {
// //                                 //     let font_id = FontSelection::Default.resolve(ui.style());
// //                                 //     let row_height = ui.fonts(|f| f.row_height(&font_id));
// //                                 //     let height = (ui.available_height() / row_height).floor();
// //                                 //     let text_editor = TextEdit::multiline(&mut multi_text).hint_text("NO").desired_rows(height as usize).desired_width(f32::INFINITY).font(FontSelection::Default);
// //                                 //     ui.add(text_editor);
// //                                 // });
// //                                 //
// //                             });
// //                         });
// //                     });
// //                 });
// //             // ScrollArea::vertical().auto_shrink([true; 2]).show_viewport(ui, |ui, viewport| {
// //             //     // ui.add(TextEdit::multiline(&mut multi_text).hint_text("NO").desired_width(f32::INFINITY));
// //             //     // ui.add(TextEdit::multiline(&mut multi_text).hint_text("NO").desired_width(f32::INFINITY));
// //             //     // ui.horizontal(|ui| {
// //             //     //     ui.add(TextEdit::multiline(&mut multi_text).hint_text("NO"));
// //             //     //     ui.add(TextEdit::multiline(&mut multi_text).hint_text("NO"));
// //             //     // });
// //             //     //let faded_color = ui
// //             //     StripBuilder::new(ui)
// //             //         //.size(Size::exact(50.0))
// //             //         //.size(Size::remainder())
// //             //         .size(Size::relative(1.0))
// //             //         //.size(Size::exact(10.0))
// //             //         .vertical(|mut strip| {
// //             //             strip.cell(|ui| {
// //             //                 ui.text_edit_multiline(&mut multi_text);
// //             //                 // ui.painter().rect_filled(
// //             //                 //     ui.available_rect_before_wrap(),
// //             //                 //     0.0,
// //             //                 //     Color32::BLUE,
// //             //                 // );
// //             //
// //             //                 //ui.add(TextEdit::multiline(&mut multi_text).desired_rows().hint_text("NO").desired_width(f32::INFINITY));
// //             //             })
// //             //         });
// //             // });
// //         });
// //     })
// //     // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
// //     //
// //     // let options = eframe::NativeOptions {
// //     //     initial_window_size: Some(egui::vec2(320.0, 240.0)),
// //     //     ..Default::default()
// //     // };
// //     //
// //     // // Our application state:
// //     // let mut name = "Arthur".to_owned();
// //     // let mut text = "".to_owned();
// //     // let mut age = 42;
// //     //
// //     // eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
// //     //     egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
// //     //         let te = egui::TextEdit::multiline(&mut text).hint_text("NO");
// //     //         ui.add(te);
// //     //         ui.text_edit_multiline(&mut text).
// //     //         ui.button("boom");
// //     //         ui.horizontal(|ui| {
// //     //             let name_label = ui.label("Your name: ");
// //     //             ui.text_edit_singleline(&mut name)
// //     //                 .labelled_by(name_label.id);
// //     //         });
// //     //         ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
// //     //         if ui.button("Click each year").clicked() {
// //     //             age += 1;
// //     //         }
// //     //         ui.label(format!("Hello '{name}', age {age}"));
// //     //     });
// //     // })
// // }
//
// // fn main() -> iced::Result {
// //     Counter::run(Settings::from(Settings {
// //         id: None,
// //         default_font: None,
// //         antialiasing: false,
// //         exit_on_close_request: true,
// //         text_multithreading: false,
// //         try_opengles_first: false,
// //         window: window::Settings {
// //             size: (1024, 720),
// //             position: Position::Centered,
// //             min_size: Some((200, 200)),
// //             max_size: None,
// //             visible: true,
// //             resizable: true,
// //             decorations: true,
// //             transparent: false,
// //             always_on_top: false,
// //             icon: None,
// //             platform_specific: PlatformSpecific,
// //         },
// //         default_text_size: 20.0,
// //         flags: (),
// //     }))
// // }
//
// #[derive(Default)]
// struct Counter {
//     text_input_search: String,
//     analyzer: String,
//     paths: Vec<PathBuf>,
//     theme: Theme,
//     value: i32,
//     index1data: Vec<Index1Data1>,
// }
//
// #[derive(Debug, Clone)]
// enum Message {
//     IncrementPressed,
//     DecrementPressed,
//     ButtonSearchPressed,
//     Selected(PathBuf),
//     TextInputSearchChanged(String),
//     TextInputAnalyzerChanged(String),
// }
//
// impl Application for Counter {
//     type Executor = executor::Default;
//     type Message = Message;
//     type Theme = Theme;
//     type Flags = ();
//
//     fn new(_flags: ()) -> (Counter, Command<self::Message>) {
//         let game_path = "/home/night/.steam/steam/steamapps/common/FINAL FANTASY XIV Online/game/sqpack/";
//         let mut file_paths: Vec<PathBuf> = Vec::new();
//         get_file_paths(game_path, &mut file_paths);
//
//         let mut index1data: Vec<Index1Data1> = Vec::new();
//         for file_path in file_paths.clone() {
//             let file_extension = file_path.extension().unwrap().to_str().unwrap();
//             if file_extension == "index" {
//                 let index_file = fs::read(file_path).unwrap();
//                 let mut index_file_buffer = Buffer::new(index_file);
//                 let index = Index::from_index1(&mut index_file_buffer);
//                 for data in index.data1 {
//                     index1data.push(data);
//                 }
//             }
//         }
//         let len = index1data.len();
//
//         (
//             Counter { index1data, value: 0, theme: Theme::Dark, paths: file_paths, analyzer: String::from(format!("{}", len)), text_input_search: String::from("yes") },
//             Command::none()
//         )
//     }
//
//
//     fn title(&self) -> String {
//         String::from("Counter - Iced")
//     }
//
//     fn update(&mut self, message: Message) -> Command<self::Message> {
//         match message {
//             Message::ButtonSearchPressed => {
//                 let index_path = IndexPath::new(&self.text_input_search);
//                 if let Ok(path) = index_path {
//                     self.analyzer = format!("{:#?}", path);
//                     for index1datum in &self.index1data {
//                         if path.index1_hash == index1datum.hash {
//                             self.analyzer.push_str(&format!("\n{}", index1datum.hash));
//                         }
//                     }
//                 } else {
//                     self.analyzer = String::from("Error parsing the path.");
//                 }
//             }
//             Message::TextInputSearchChanged(value) => {
//                 self.text_input_search = value
//             }
//             Message::TextInputAnalyzerChanged(value) => {
//                 self.analyzer = value
//             }
//             Message::IncrementPressed => {
//                 self.value += 1
//             }
//             Message::DecrementPressed => {
//                 self.value -= 1
//             }
//             Message::Selected(path) => {
//                 if path.is_file() && path.extension().unwrap().to_str().unwrap() == "index" {
//                     let path_str = path.to_str().unwrap();
//                     let file = fs::read(path_str);
//                     if let Ok(file) = file {
//                         let mut buffer = Buffer::new(file);
//                         let metadata = parser::ffxiv::index::Index::from_index1(&mut buffer);
//                         let mut analizer = String::new();
//                         for data in metadata.data1 {
//                             analizer.push_str(format!("{}\n", data.hash).as_str())
//                         }
//                         self.analyzer = analizer
//                     } else if let Err(err) = file {
//                         self.analyzer = String::from(format!("Error reading file: {} \n{}", path_str, err.to_string()))
//                     }
//                 } else {
//                     self.analyzer = String::from(format!("Invalid file: {}", path.to_str().unwrap()))
//                 }
//             }
//         }
//
//         Command::none()
//     }
//
//     fn view(&self) -> Element<Message> {
//         container(column![
//             row![
//                 text_input("Hello", &self.text_input_search).on_input(Message::TextInputSearchChanged).width(Length::Fill),
//                 button("Search").on_press(Message::ButtonSearchPressed).width(200)
//             ],
//             row![
//                 scrollable((&self.paths).iter().map(|path| -> Element<Message> {
//                     let file_name = path.file_name().unwrap().to_str().unwrap();
//                     let file_extension = path.extension().unwrap().to_str().unwrap();
//                     Element::from(button(file_name).on_press(Message::Selected(path.clone())).padding(10).width(Length::Fill))
//                 }).fold(Column::new().spacing(10), |a, b| a.push(b))).width(200),
//                 scrollable(column![
//                     text_input("Nothing!", &self.analyzer).on_input(Message::TextInputAnalyzerChanged),
//                 ]).width(Length::Fill).height(Length::Fill)
//             ]
//         ]).width(Length::Fill)
//             .height(Length::Fill)
//             .into()
//     }
//
//     fn theme(&self) -> Theme { self.theme.clone() }
// }
//
//
// fn decode_zlib() {
//     let file = fs::read("./0").unwrap();
//     let mut outt: Vec<u8> = Vec::with_capacity(204455);
//     let mut y = Decompress::new(false);
//     y.decompress_vec(&file, &mut outt, FlushDecompress::Finish).unwrap();
//     fs::write("./l0", outt);
// }
//
// fn decode_audio() {
//     let mut buffer = reader::Buffer::from_file(FILE_PATH);
//     let metadata = parser::audio::sqex_scd::Metadata::new(&mut buffer);
//     if metadata.entry_channels > 2 || metadata.entry_codex != 12 || metadata.entry_wave_format_ex != 7 {
//         panic!("Unsupported format");
//     }
//     let decoded = decoder::sqex_scd::decode(&metadata, &mut buffer);
//
//     let spec = hound::WavSpec {
//         channels: metadata.entry_channels as u16,
//         sample_rate: metadata.entry_sample_rate as u32,
//         bits_per_sample: 16,
//         sample_format: hound::SampleFormat::Int,
//     };
//     let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
//     for index in (0..decoded.len()).step_by(2) {
//         writer.write_sample(i16::from_le_bytes([decoded[index], decoded[index + 1]])).unwrap();
//     }
// }
