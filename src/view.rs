use crate::panels::*;
use crate::NB;
use sdl2::event::{Event, EventType};
use sdl2::keyboard::Keycode;
use std::{thread, time};

pub const SIZE: i32 = 50;
pub const DELTA: i32 = 2 * SIZE;
pub const HEIGHT: i32 = NB * SIZE;
pub const BOARD_WIDTH: i32 = NB * SIZE;
pub const WIDTH: i32 = 2 * BOARD_WIDTH + DELTA;
pub const OFFSET_X: i32 = BOARD_WIDTH + 100;

pub fn run() -> Result<(), String> {
    let sdl_context = try_string!(sdl2::init());
    let video_subsystem = try_string!(sdl_context.video());
    //    let event_subsystem = sdl_context.event().unwrap();
    //    let audio_subsystem = sdl_context.audio().unwrap();
    //    let timer_subsystem = sdl_context.timer().unwrap();

    let window = try_string!(video_subsystem
        .window("Rust Battleship", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build());

    let mut canvas = try_string!(window.into_canvas().build());
    let mut event_pump = sdl_context.event_pump()?;

    event_pump.enable_event(EventType::AppTerminating);
    event_pump.enable_event(EventType::MouseButtonUp);
    event_pump.enable_event(EventType::KeyUp);
    event_pump.enable_event(EventType::Quit);

    let mut panel: Option<Box<Panel>> = None;
    panel = Some(Box::new(MenuPanel::new(
        &mut panel as *mut Option<Box<Panel>>,
    )));

    loop {
        canvas.clear();
        if let Some(panel) = &mut panel {
            panel.render(&mut canvas)?;
            if panel.do_loop()? {
                continue;
            }
        }
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::AppTerminating { .. }
                | Event::KeyUp {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(1),
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    panel = Some(Box::new(MenuPanel::new(
                        &mut panel as *mut Option<Box<Panel>>,
                    )))
                }
                e => {
                    if let Some(panel) = &mut panel {
                        if panel.manage_event(e)? {
                            continue;
                        }
                    }
                }
            };
        }

        thread::sleep(time::Duration::from_millis(50));
    }
}
