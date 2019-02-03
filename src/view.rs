use crate::game::{Game, GameType};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::{thread, time};

const NB: i32 = 12;
const SIZE: i32 = 50;
const DELTA: i32 = 100;
const HEIGHT: i32 = NB * SIZE;
const BOARD_WIDTH: i32 = NB * SIZE;
const WIDTH: i32 = 2 * BOARD_WIDTH + DELTA;
const OFFSET_X: i32 = BOARD_WIDTH + 100;

pub fn view_thread(mutex: Arc<Mutex<GameType>>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rust Battleship", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    loop {
        thread::sleep(time::Duration::from_millis(100));
        canvas.clear();
        let mut gt = mutex.lock().unwrap();

        match gt.deref_mut() {
            GameType::Network(game, _) => game,
            GameType::_Ai(game, _) => game,
        }
        .print(&mut canvas)
        .unwrap();
        canvas.present();
    }
}

pub fn fill_circle(
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

impl Game {
    pub fn print(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(None).unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for i in 0..NB + 1 {
            canvas.draw_line((0, i * SIZE), (BOARD_WIDTH, i * SIZE))?;
            canvas.draw_line((OFFSET_X, i * SIZE), (WIDTH, i * SIZE))?;

            canvas.draw_line((i * SIZE, 0), (i * SIZE, BOARD_WIDTH))?;
            canvas.draw_line((OFFSET_X + i * SIZE, 0), (OFFSET_X + i * SIZE, BOARD_WIDTH))?;
        }

        let healthy_boat = Color::RGB(0, 0, 255);
        let wounded_boat = Color::RGB(255, 0, 0);

        for boat in &self.player.boats {
            let dx = boat.direction.dx() as i32;
            let dy = boat.direction.dy() as i32;
            let mut x = boat.position.0 as i32;
            let mut y = boat.position.1 as i32;

            for l in &boat.detailed_life {
                fill_circle(
                    canvas,
                    if *l { healthy_boat } else { wounded_boat },
                    SIZE / 2 + x as i32 * SIZE,
                    SIZE / 2 + y as i32 * SIZE,
                    SIZE / 3,
                )?;

                x += dx;
                y += dy;
            }
        }

        for j in 0..self.shot_boats.len() {
            for i in 0..self.shot_boats[j].len() {
                match self.shot_boats[j][i] {
                    None => (),
                    Some(b) => fill_circle(
                        canvas,
                        if b { healthy_boat } else { wounded_boat },
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
