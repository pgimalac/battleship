use crate::game::GameType;
use crate::game_view::GameRenderer;
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

pub const SIZE: i32 = 50;
pub const DELTA: i32 = 2 * SIZE;
pub const HEIGHT: i32 = NB * SIZE;
pub const BOARD_WIDTH: i32 = NB * SIZE;
pub const WIDTH: i32 = 2 * BOARD_WIDTH + DELTA;
pub const OFFSET_X: i32 = BOARD_WIDTH + 100;

pub fn view_thread(mut mutex: Arc<Mutex<GameType>>, mut work: Arc<AtomicBool>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    //    let event_subsystem = sdl_context.event().unwrap();
    //    let audio_subsystem = sdl_context.audio().unwrap();
    //    let timer_subsystem = sdl_context.timer().unwrap();

    //    let ttf_context = sdl2::ttf::init().unwrap();
    let window = video_subsystem
        .window("Rust Battleship", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    /*    let font = ttf_context
            .load_font("../assets/OpenDyslexic-Regular.ttf", 16)
            .unwrap()
            .render("Hello World !")
            .solid(Color::RGB(42, 42, 42))
            .unwrap();
    */
    //    canvas.render(font);

    let mut event_pump = sdl_context.event_pump().unwrap();

    event_pump.enable_event(EventType::AppTerminating);
    event_pump.enable_event(EventType::MouseButtonUp);
    event_pump.enable_event(EventType::KeyUp);
    event_pump.enable_event(EventType::Quit);

    while work.load(Ordering::Relaxed) {
        canvas.clear();

        canvas.print_game_type(&*mutex.lock().unwrap()).unwrap();
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
