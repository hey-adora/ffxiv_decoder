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


fn main() {
    let ffxiv = FFXIV::new("/mnt/hdd2/.local/share/Steam/steamapps/common/FINAL FANTASY XIV Online");
    //let exh = ffxiv.get_asset_from_path("exd/action.exh").unwrap().to_exh();
    //let exd =  ffxiv.get_asset_from_path("exd/action_0_en.exd").unwrap().to_exd(&exh);
    let tex =  ffxiv.get_asset_from_path("chara/equipment/e0171/texture/v01_c0101e0171_glv_n.tex").unwrap();
    ffxiv.save_all();




    println!("test");


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