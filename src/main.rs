extern crate sdl2;

use sdl2::VideoSubsystem;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

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
        .build()
        .unwrap()
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = create_window(&video_subsystem, "Game Of Life");
    let mut canvas = create_canvas(window);

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::F3), .. } => {
                    println!("Toggle FPS");
                }
                _ => {},
            }
        }
    }
}
