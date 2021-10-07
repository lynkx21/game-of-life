extern crate sdl2;

use sdl2::VideoSubsystem;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::ttf::Font;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::video::{Window, WindowContext};
use std::path::Path;

const SCREEN_WIDTH:  u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn create_window(video_subsystem: &VideoSubsystem, title: &str) -> Window {
    video_subsystem.window(title, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap()
}

fn create_canvas(window: Window) -> Canvas<Window> {
    window.into_canvas()
        .present_vsync()
        .build()
        .unwrap()
}

pub fn main() {
    // Initialize
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();
    let perf_freq = timer_subsystem.performance_frequency();

    // Rendering bindings
    let window = create_window(&video_subsystem, "Game Of Life");
    let mut canvas = create_canvas(window);
    let texture_creator = canvas.texture_creator();

    // Font bindings
    let font_path = Path::new("./src/fonts/MesloLGS NF Regular.ttf");
    let font_fps = ttf_context.load_font(font_path, 14).unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_perf_counter = timer_subsystem.performance_counter();
    let mut toggle_fps = false;

    // Game Loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::F3), .. } => {
                    toggle_fps = !toggle_fps;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    println!("Reset");
                }
                _ => {},
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // FPS CALCULATIONS
        let end_perf_counter = timer_subsystem.performance_counter();
        let perf_counter_elapsed = end_perf_counter - last_perf_counter;
        let mspf = 1_000f32 * perf_counter_elapsed as f32 / perf_freq as f32;
        let fps = perf_freq as f32 / perf_counter_elapsed as f32;
        last_perf_counter = end_perf_counter;
        if toggle_fps {
            let (fps_texture, fps_rect) = show_fps(&texture_creator, &font_fps, mspf, fps);
            canvas.copy(&fps_texture, None, Some(fps_rect)).unwrap(); // FONT STUFF
        }

        canvas.present();


    } // 'running loop
}

fn show_fps<'a>(texture_creator: &'a TextureCreator<WindowContext>, font_fps: &Font, mspf: f32, fps: f32) -> (Texture<'a>, Rect) {
    let fps_string = format!("ms/f: {:.3}, fps: {:.3}", mspf, fps);
    let surface = font_fps
        .render(&fps_string)
        .blended(Color::GREEN)
        .unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    let texture_rect = Rect::new((SCREEN_WIDTH - width) as i32, 0, width, height);
    (texture, texture_rect)
}