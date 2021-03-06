mod buttons;
mod connection;
mod creation;
mod end_game;
mod game;
mod game_renderer;
mod menu;
mod panel;

use crate::{
    view::{
        game::{BOARD_WIDTH, DELTA},
        menu::MenuPanel,
        panel::Panel,
    },
    NB,
};
use sdl2::{
    event::{Event, EventType},
    keyboard::Keycode,
    mouse::MouseState,
};
use std::{thread, time};

pub const HEIGHT: i32 = BOARD_WIDTH;
pub const WIDTH: i32 = 2 * BOARD_WIDTH + DELTA;

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init().map_err(|x| x.to_string())?;
    let video_subsystem = sdl_context.video().map_err(|x| x.to_string())?;
    //    let event_subsystem = sdl_context.event().unwrap();
    //    let audio_subsystem = sdl_context.audio().unwrap();
    //    let timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem
        .window("Rust Battleship", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|x| x.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|x| x.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    event_pump.disable_event(EventType::MouseMotion);
    event_pump.enable_event(EventType::AppTerminating);
    event_pump.enable_event(EventType::MouseButtonUp);
    event_pump.enable_event(EventType::MouseButtonDown);
    event_pump.enable_event(EventType::KeyUp);
    event_pump.enable_event(EventType::KeyDown);
    event_pump.enable_event(EventType::Quit);

    let mut panel: Box<Panel> = Box::new(MenuPanel::new());

    loop {
        canvas.clear();
        panel.render(&mut canvas, MouseState::new(&event_pump))?;
        canvas.present();
        if let Some(newpanel) = panel.do_loop()? {
            panel = newpanel;
            continue;
        }

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
                    panel = Box::new(MenuPanel::new());
                    continue;
                }
                e => {
                    if let Some(newpanel) = panel.manage_event(e)? {
                        panel = newpanel;
                        continue;
                    }
                }
            };
        }

        thread::sleep(time::Duration::from_millis(30));
    }
}
