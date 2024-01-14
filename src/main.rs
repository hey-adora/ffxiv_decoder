use std::{path::Path};
use std::time::Instant;
use glium::{Display, Surface};
use game_data_resolver::ffxiv::FFXIV;
use glium::glutin::{self, event::{Event, WindowEvent}, event_loop::{EventLoop, ControlFlow}, window::WindowBuilder};
use imgui::{Context, ClipboardBackend, FontSource, FontConfig, FontGlyphRanges, Ui, Condition};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use copypasta::{ClipboardContext, ClipboardProvider};


fn main() {
    let ffxiv = FFXIV::new("/mnt/hdd2/.local/share/Steam/steamapps/common/FINAL FANTASY XIV Online");
    let exh = ffxiv.get_asset_from_path("exd/action.exh").unwrap().to_exh();
    println!("test");
    let system = System::init("Hello");
    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];
    system.main_loop(move |_, ui| {
        ui.window("Hello wow").size([300.0, 110.0], Condition::FirstUseEver).build(|| {
            ui.text_wrapped("Hello d");
            ui.text_wrapped("aaaaaaaa");
            if ui.button(choices[value]) {
                value += 1;
                value %= 2;
            }
            ui.button("this...is...imgui-rs");
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!("Mouse Position: ({:.1}, {:.1})", mouse_pos[0], mouse_pos[1]));
        });
    })
    //let system = supp
    // ffxiv.get_asset_from_path("exd/action.exh").unwrap().save_decompressed("./media/1.exh");
    // ffxiv.get_asset_from_path("exd/action_0_en.exd").unwrap().save_decompressed("./media/2.exh");
}

pub struct ClipboardSupport(pub ClipboardContext);

impl ClipboardSupport {
    pub fn new() -> Option<ClipboardSupport> {
        ClipboardContext::new().ok().map(ClipboardSupport)
    }
}

impl ClipboardBackend for ClipboardSupport {
    fn get(&mut self) -> Option<String> {
        self.0.get_contents().ok()
    }

    fn set(&mut self, text: &str) {
        // ignore errors?
        let _ = self.0.set_contents(text.to_owned());
    }
}


pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32
}

impl System {
    pub fn init(title: &str) -> System{
        let title = match Path::new(title).file_name() {
            Some(file_name) => file_name.to_str().unwrap(),
            None => title
        };
        let event_loop = EventLoop::new();
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let builder = WindowBuilder::new().with_title(title.to_owned()).with_inner_size(glutin::dpi::LogicalSize::new(1024f64, 768f64));
        let display = Display::new(builder, context, &event_loop).expect("Failed to initialize display");

        let mut imgui = Context::create();
        imgui.set_ini_filename(None);

        if let Some(backend) = ClipboardSupport::new() {
            imgui.set_clipboard_backend(backend);
        } else {
            eprintln!("Failed to initialize clipboard");
        }

        let mut platform = WinitPlatform::init(&mut imgui);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();

            let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
                match factor.parse::<f64>() {
                    Ok(f) => HiDpiMode::Locked(f),
                    Err(e) => panic!("Invalid scaling factor: {}", e)
                }
            } else {
                HiDpiMode::Default
            };

            platform.attach_window(imgui.io_mut(), window, dpi_mode);
        }

        let font_size = 13.0;

        imgui.fonts().add_font(&[
            FontSource::TtfData {
                data: include_bytes!("../resources/Roboto-Regular.ttf"),
                size_pixels: font_size,
                config: Some(FontConfig {
                    rasterizer_multiply: 1.5,
                    oversample_h: 4,
                    oversample_v: 4,
                    ..FontConfig::default()
                })
            },
            FontSource::TtfData {
                data: include_bytes!("../resources/mplus-1p-regular.ttf"),
                size_pixels: font_size,
                config: Some(FontConfig {
                    oversample_h: 4,
                    oversample_v: 4,
                    glyph_ranges: FontGlyphRanges::japanese(),
                    ..FontConfig::default()
                })
            }
        ]);

        let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

        System {
            event_loop,
            display,
            imgui,
            platform,
            renderer,
            font_size
        }

    }

    pub fn main_loop<F: FnMut(&mut bool, &mut Ui) + 'static>(self, mut run_ui: F) {
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;

        let mut last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow | match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform.prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let ui = imgui.frame();

                let mut run = true;
                run_ui(&mut run, ui);
                if !run {
                    *control_flow = ControlFlow::Exit
                }

                let gl_window = display.gl_window();
                let mut target = display.draw();
                target.clear_color_srgb(0.0, 0.0, 0.0, 0.0);
                platform.prepare_render(ui, gl_window.window());
                let draw_data = imgui.render();
                renderer.render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        });
    }
}