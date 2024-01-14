use std::{fs, path::Path};
use std::fs::create_dir_all;
use std::os::raw::c_char;
use std::path::PathBuf;
use std::thread::scope;
use std::time::Instant;
use glium::{Display, Surface};
use game_data_resolver::ffxiv::FFXIV;
use glium::glutin::{self, event::{Event, WindowEvent}, event_loop::{EventLoop, ControlFlow}, window::WindowBuilder};
use imgui::{sys, Direction, Context, ClipboardBackend, FontSource, FontConfig, FontGlyphRanges, Ui, Condition};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use copypasta::{ClipboardContext, ClipboardProvider};
use imgui::sys::{igGetCurrentWindow, ImGuiDockNodeFlags};
use eframe::{egui, Frame};
use game_data_resolver::ffxiv::asset_exh_file::{AssetEXHFileColumnKind, AssetEXHFileLanguage};
use game_data_resolver::ffxiv::ffxiv_asset::{FFXIVAssetParserDat, FFXIVAssetParserDatBlock, StandardFile, TextureFile, FFXIVAssetPathDat, FileType};
use game_data_resolver::ffxiv::ffxiv_buffer::{FFXIVBuffer, FFXIVBufferFile};
use game_data_resolver::ffxiv::ffxiv_game::FFXIVAssetFiles;


fn contains_bytes(source: &Vec<u8>, sample: &Vec<u8>) -> bool {
    let mut found = false;
    let mut contains = 0;
    let contains_max = sample.len();
    for src_byte in source {
        if *src_byte == sample[contains] {
            contains += 1;
            if contains == contains_max {
                found = true;
                break;
            }
        }
        else {
            contains = 0;
        }
    }
    found
}

fn main() {
     let ffxiv = FFXIVAssetFiles::new("/mnt/hdd2/.local/share/Steam/steamapps/common/FINAL FANTASY XIV Online");
    // //let exh = ffxiv.get_asset_from_path("exd/action.exh").unwrap().to_exh();
    // //let exd =  ffxiv.get_asset_from_path("exd/action_0_en.exd").unwrap().to_exd(&exh);
    // let tex =  ffxiv.get_asset_from_path("chara/equipment/e0171/texture/v01_c0101e0171_glv_n.tex").unwrap();
    // ffxiv.save_all();
    //
    //
    //
    //
    // println!("test"); // chara/equipment/e0171/texture/v01_c0101e0171_glv_n.tex // exd/root.exl

    //let paths = ffxiv_files.get_assets_path_from_file("/home/night/Documents/GitHub/game_data_resolver/media2/all_paths.txt");
    //let dat_items = ffxiv_files.get_index1_dat_items();
    // let tex = ffxiv_files.get_asset("chara/equipment/e0171/texture/v01_c0101e0171_glv_n.tex").unwrap().decompress().unwrap();
    // let exl = ffxiv_files.get_asset("exd/root.exl").unwrap().decompress().unwrap();
    // fs::write("./v01_c0101e0171_glv_n.tex", tex).unwrap();
    // fs::write("./root.exl2", exl).unwrap();

    //let mut buffer = FFXIVBuffer::from_file_path(&dat_file_path.path);
    //let tex = TextureFile::new(&mut buffer, index.data_file_offset);

    // let source = vec![1, 2, 3, 4, 5];
    // let sample = vec![2, 3, 4];
    //
    // let mut found = contains_bytes(&source, &sample);
    // if found {
    //     println!("{}", "TRUE");
    // }


    // let names = ffxiv.get_paths("/home/night/Documents/GitHub/game_data_resolver/media2/all_paths.txt");
    //
    // let mut output = String::new();
    // for (hash, name) in names {
    //     output.push_str(&format!("{} {}\n", hash.to_string(), name.path_str));
    // }
    // fs::write("./test.txt", output).unwrap();


    // let files = fs::read_dir("/home/night/Downloads/0000000000000000000").unwrap();
    // let files2 = fs::read_dir("/home/night/Downloads/0000000000000000000").unwrap();
    // let sample = (b"Are you just seeking to pass the time").to_vec();
    //
    // let max_index: f32 = files2.count() as f32;
    // let check_every: f32 = (max_index / 100.0).floor();
    // for (index, file) in files.enumerate() {
    //     let path = file.unwrap().path().to_str().unwrap().to_owned();
    //     let source = fs::read(&path).unwrap();
    //
    //     let found = contains_bytes(&source, &sample);
    //     if found {
    //         println!("{}", path);
    //     }
    //
    //     let index = index as f32;
    //     if index % check_every == 0.0 {
    //         let done = (index / max_index) * 100.0;
    //         println!("Searching for str: {}\n", done);
    //     }
    // }




    // let ffxiv_files = FFXIVAssetFiles::new("/home/night/Games/FINAL FANTASY XIV Online/FINAL FANTASY XIV Online");

    //ffxiv.export_all_text("/home/night/Downloads/00000000000000000002", "/home/night/Documents/GitHub/game_data_resolver/media2/all_paths.txt" );
    let asset = ffxiv.get_asset("exd/custom/000/regseaarmguild_00056_0_en.exd");

    println!("test");


    //let compressed_asset = FFXIVAssetParserDat::from_file_path(dat_file_path.path, index.data_file_offset);
    //let decompressed_data_blocks = compressed_asset.to_decompressed();
    //fs::write("./test9.exl", decompressed_data_blocks.concat()).unwrap();


    // env_logger::init();
    // let options = eframe::NativeOptions {
    //     initial_window_size: Some(egui::vec2(1024.0, 768.0)),
    //     ..Default::default()
    // };
    // eframe::run_native(
    //     "FFXIV Pathfinder",
    //     options,
    //     Box::new(|_cc| Box::<MyApp>::default())
    // )
}

struct MyApp {
    name: String,
    age: u32
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "night".to_owned(),
            age: 69
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hello");
        });
    }
}