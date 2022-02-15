use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};

use std::path::PathBuf;
use winit::dpi::PhysicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use log::{info, warn};
use notify::{raw_watcher, Op, RecursiveMode, Watcher};
use pixels::wgpu::Color;
use std::sync::mpsc::channel;
use std::thread;

use tiny_skia::Pixmap;
use usvg::{Options, Tree};

struct State {
    file: PathBuf,

    options: Options,
    pixels: Pixmap,
    svg_data: Tree,

    width: u32,
    height: u32,
}

fn main() -> Result<()> {
    // INFRA
    pretty_env_logger::init();

    // CLI
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n\tsvgview <path-to-svg>");
        std::process::exit(0);
    }
    let file = std::fs::canonicalize(&args[1]).expect("Failed to interpret argument as path!");

    // DISPLAY WINDOW
    let event_loop = EventLoop::<()>::with_user_event();
    let mut input = WinitInputHelper::new();
    let window = {
        WindowBuilder::new()
            .with_title("svgview")
            .with_resizable(true)
            .build(&event_loop)
            .unwrap()
    };

    // APPLICATION STATE
    let window_size = window.inner_size();
    let mut state = State::new(file.clone(), window_size);

    // PIXEL BUFFER
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture)?
    };
    pixels.set_clear_color(Color::WHITE);

    // FILE WATCHER
    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).expect("Could not create filesystem watcher!");
    watcher
        .watch(file, RecursiveMode::NonRecursive)
        .expect("Could not start filesystem watcher!");

    let evp = event_loop.create_proxy();
    thread::spawn(move || loop {
        match rx.recv() {
            Ok(event) => {
                if let Ok(Op::CLOSE_WRITE) = event.op {
                    evp.send_event(())
                        .expect("Failed to notify UI of file write!");
                }
            }
            Err(e) => warn!("watch error: {:?}", e),
        }
    });

    // INTERFACE EVENT LOOP
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // rasterize the SVG and copy the data to the pixel buffer
            let pixel_buffer = pixels.get_frame();
            pixel_buffer.copy_from_slice(state.pixels.data());

            if pixels
                .render()
                .map_err(|e| warn!("Rendering failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if let Event::UserEvent(_) = event {
            state.handle_file_change();
            window.request_redraw();
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                // resize pixel buffer, resize surface buffer, resize SVG buffer, then redraw
                pixels.resize_buffer(size.width, size.height);
                pixels.resize_surface(size.width, size.height);
                state.resize(size.width, size.height);
                window.request_redraw();
            }
        }
    });
}

impl State {
    fn new(file: PathBuf, window_size: PhysicalSize<u32>) -> Self {
        let mut opt = usvg::Options {
            resources_dir: file.parent().map(|p| p.to_path_buf()),
            ..Default::default()
        };
        opt.fontdb.load_system_fonts();

        let file_data = std::fs::read(&file).expect("Could not read input file!");
        let svg_data =
            usvg::Tree::from_data(&file_data, &opt.to_ref()).expect("Could not parse data as SVG!");

        let mut state = Self {
            file,
            width: window_size.width,
            height: window_size.height,

            options: opt,
            pixels: Pixmap::new(window_size.width, window_size.height)
                .expect("Could not allocate memory for display!"),
            svg_data,
        };
        state.rasterize_svg();
        state
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.pixels =
            Pixmap::new(self.width, self.height).expect("Could not allocate memory for display!");
        self.rasterize_svg();
    }

    fn handle_file_change(&mut self) {
        let svg_data = std::fs::read(&self.file).expect("Could not read input file!");
        self.svg_data = usvg::Tree::from_data(&svg_data, &self.options.to_ref())
            .expect("Could not parse data as SVG!");
        self.rasterize_svg();
    }

    fn rasterize_svg(&mut self) {
        self.pixels
            .data_mut()
            .copy_from_slice(&vec![0; self.width as usize * self.height as usize * 4]);
        resvg::render(
            &self.svg_data,
            usvg::FitTo::Size(self.width, self.height),
            tiny_skia::Transform::default(),
            self.pixels.as_mut(),
        )
        .expect("Could not rasterize SVG!");
    }
}
