use crate::boat::Boat;
use crate::game::{Game, GameType};
use crate::quit;
use crate::NB;
use sdl2::event::{Event, EventType};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

const SIZE: i32 = 50;
const DELTA: i32 = 2 * SIZE;
const HEIGHT: i32 = NB * SIZE;
const BOARD_WIDTH: i32 = NB * SIZE;
const WIDTH: i32 = 2 * BOARD_WIDTH + DELTA;
const OFFSET_X: i32 = BOARD_WIDTH + 100;

const HEALTHY_BOAT: Color = Color {
    r: 0,
    g: 0,
    b: 255,
    a: 0,
};
const WOUNDED_BOAT: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 0,
};

pub fn view_thread(mut mutex: Arc<Mutex<GameType>>, mut work: Arc<AtomicBool>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rust Battleship", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    event_pump.enable_event(EventType::AppTerminating);
    event_pump.enable_event(EventType::MouseButtonUp);
    event_pump.enable_event(EventType::KeyUp);
    event_pump.enable_event(EventType::Quit);

    let mut start_time: SystemTime;
    let wait_time = Duration::from_millis(100);

    while work.load(Ordering::Relaxed) {
        start_time = SystemTime::now();

        canvas.clear();

        mutex.lock().unwrap().print(&mut canvas).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::AppTerminating { .. }
                | Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Escape),
                    ..
                } => quit(&mut mutex, &mut work),
                Event::MouseButtonUp { x, y, .. } => {
                    println!("Mouse button up");
                    if x >= OFFSET_X && y >= 0 && x < WIDTH && y < HEIGHT {
                        let x = ((x - OFFSET_X) / SIZE) as u8;
                        let y = (y / SIZE) as u8;
                        mutex.lock().unwrap().attack((x, y));
                    }
                }
                _ => {}
            }
        }

        match start_time.elapsed() {
            Ok(t) => {
                if t < wait_time {
                    thread::sleep(wait_time - t);
                }
            }
            Err(_) => thread::sleep(wait_time),
        }
    }
}

fn fill_circle(
    canvas: &mut Canvas<Window>,
    color: Color,
    x: i32,
    y: i32,
    r: i32,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    for i in 0..r {
        let mut j = 0;
        while i >= j && (i * i) + (j * j) <= (r * r) {
            canvas.draw_point(Point::new(x + i, y + j))?;
            canvas.draw_point(Point::new(x + i, y - j))?;
            canvas.draw_point(Point::new(x - i, y + j))?;
            canvas.draw_point(Point::new(x - i, y - j))?;
            canvas.draw_point(Point::new(x + j, y + i))?;
            canvas.draw_point(Point::new(x + j, y - i))?;
            canvas.draw_point(Point::new(x - j, y + i))?;
            canvas.draw_point(Point::new(x - j, y - i))?;

            j += 1;
        }
    }
    Ok(())
}

impl GameType {
    pub fn print(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(None).unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for i in 0..NB + 1 {
            canvas.draw_line((0, i * SIZE), (BOARD_WIDTH, i * SIZE))?;
            canvas.draw_line((OFFSET_X, i * SIZE), (WIDTH, i * SIZE))?;

            canvas.draw_line((i * SIZE, 0), (i * SIZE, BOARD_WIDTH))?;
            canvas.draw_line((OFFSET_X + i * SIZE, 0), (OFFSET_X + i * SIZE, BOARD_WIDTH))?;
        }
        self.get_main_game().print(canvas)
    }
}

impl Game {
    fn print(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for boat in &self.player.boats {
            boat.print(canvas)?;
        }

        for i in 0..self.shot_boats.len() {
            for j in 0..self.shot_boats[i].len() {
                match self.shot_boats[i][j] {
                    None => (),
                    Some(b) => fill_circle(
                        canvas,
                        if b { WOUNDED_BOAT } else { HEALTHY_BOAT },
                        OFFSET_X + SIZE / 2 + i as i32 * SIZE,
                        SIZE / 2 + j as i32 * SIZE,
                        SIZE / 3,
                    )?,
                }
            }
        }

        Ok(())
    }
}

impl Boat {
    fn print(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let dx = self.direction.dx() as i32;
        let dy = self.direction.dy() as i32;
        let mut x = self.position.0 as i32;
        let mut y = self.position.1 as i32;

        for l in &self.detailed_life {
            fill_circle(
                canvas,
                if *l { HEALTHY_BOAT } else { WOUNDED_BOAT },
                SIZE / 2 + x as i32 * SIZE,
                SIZE / 2 + y as i32 * SIZE,
                SIZE / 3,
            )?;

            x += dx;
            y += dy;
        }
        Ok(())
    }
}
