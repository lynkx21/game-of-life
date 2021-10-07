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
use rand::Rng;

const SCREEN_WIDTH:   u32 = 800;
const SCREEN_HEIGHT:  u32 = 600;
const VIRTUAL_WIDTH:  u32 = 40;
const VIRTUAL_HEIGHT: u32 = 30;
const CELL_SIZE:      u32 = 20;
const ALIVE_COLOR: Color = Color::WHITE;
// const DEAD_COLOR:  Color = Color::BLACK;

#[derive(Debug)]
struct Grid {
    tiles: [[u8; VIRTUAL_HEIGHT as usize]; VIRTUAL_WIDTH as usize],
    n_rows: usize,
    n_cols: usize,
}

impl Grid {
    fn new() -> Grid {
        const N_ROWS: usize = VIRTUAL_WIDTH as usize;
        const N_COLS: usize = VIRTUAL_HEIGHT as usize;
        let tiles = [[0u8; N_COLS]; N_ROWS];
        Grid { tiles, n_rows: N_ROWS, n_cols: N_COLS }
    }

    fn reset(&mut self) {
        for i in 0..self.n_rows {
            for j in 0..self.n_cols {
                let cell_status: u8 = rand::thread_rng().gen_range(0..=1);
                self.tiles[i][j] = cell_status;
            }
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        let mut cells: Vec<Rect> = vec![];
        for i in 0..self.n_rows {
            for j in 0..self.n_cols {
                if let 1 = self.tiles[i][j] {
                    let cell = Rect::new(
                        i as i32 * CELL_SIZE as i32,
                        j as i32 * CELL_SIZE as i32,
                        CELL_SIZE,
                        CELL_SIZE
                    );
                    cells.push(cell);
                }
            }
        }
        canvas.set_draw_color(ALIVE_COLOR);
        canvas.fill_rects(&cells).unwrap();
    }
}

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

    // Cell grid
    let mut grid = Grid::new();
    grid.reset();

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
                    grid.reset();
                }
                _ => {},
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        grid.render(&mut canvas);

        // FPS CALCULATIONS
        let end_perf_counter = timer_subsystem.performance_counter();
        let perf_counter_elapsed = end_perf_counter - last_perf_counter;
        let mspf = 1_000f32 * perf_counter_elapsed as f32 / perf_freq as f32;
        let fps = perf_freq as f32 / perf_counter_elapsed as f32;
        last_perf_counter = end_perf_counter;
        if toggle_fps {
            let (fps_texture, fps_rect) = show_fps(&texture_creator, &font_fps, mspf, fps);
            canvas.set_draw_color(Color::BLACK);
            canvas.fill_rect(fps_rect).unwrap();
            canvas.copy(&fps_texture, None, Some(fps_rect)).unwrap(); // FONT STUFF
        }

        canvas.present();


    } // 'running loop
}
