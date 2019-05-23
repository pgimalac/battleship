// multiplayer connection panel
use crate::network::{create_host_socket, find_host, wait_client};
use crate::utils::*;
use crate::view::{
    buttons::Button,
    creation::CreationPanel,
    panel::{Panel, TEXT_COLOR},
};
use sdl2::{
    event::{
        Event,
        Event::{KeyUp, MouseButtonUp},
    },
    keyboard::Keycode,
    mouse::{MouseButton, MouseState},
    render::Canvas,
    video::Window,
};
use std::{net::TcpListener, time::Duration};

pub struct ConnectPanel {
    buttons: Vec<Button>,
    connect_button: Button,
    host_button: Button,
    address: String,
    host_socket: Option<TcpListener>,
}

impl ConnectPanel {
    pub fn new() -> Self {
        let address = String::with_capacity(39);
        println!("Creation of the connect panel");
        ConnectPanel {
            buttons: vec![],
            connect_button: Button::new(
                0,
                0,
                100,
                100,
                YELLOW,
                "Connect".to_string(),
                TEXT_COLOR,
                Box::new(|| None),
            ),
            host_button: Button::new(
                0,
                100,
                100,
                100,
                MAGENTA,
                "Connect".to_string(),
                TEXT_COLOR,
                Box::new(|| None),
            ),
            address,
            host_socket: None,
        }
    }
}

impl Panel for ConnectPanel {
    fn button_vec_mut(&mut self) -> &mut Vec<Button> {
        &mut self.buttons
    }

    fn button_vec(&self) -> &Vec<Button> {
        &self.buttons
    }

    fn manage_event(&mut self, event: Event) -> Result<Option<Box<Panel>>, String> {
        match event {
            MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                for button in &mut self.buttons {
                    if button.contains_point((x, y)) {
                        if let Some(panel) = button.execute() {
                            return Ok(Some(panel));
                        }
                    }
                }
                if self.connect_button.contains_point((x, y)) {
                    match find_host(self.address.as_str()) {
                        Ok(sock) => {
                            sock.set_read_timeout(Some(Duration::from_nanos(1)))
                                .map_err(|x| x.to_string())?;
                            return Ok(Some(Box::new(CreationPanel::new(Some((sock, false))))));
                        }
                        Err(e) => {
                            println!("{}", e.to_string());
                        }
                    }
                }
                if self.host_button.contains_point((x, y)) {
                    if let None = self.host_socket {
                        self.host_socket = Some(create_host_socket()?);
                    }
                }
            }
            KeyUp {
                keycode: Some(keycode),
                ..
            } => {
                if let Some(s) = match keycode {
                    Keycode::Num0 | Keycode::Kp0 => Some('0'),
                    Keycode::Num1 | Keycode::Kp1 => Some('1'),
                    Keycode::Num2 | Keycode::Kp2 => Some('2'),
                    Keycode::Num3 | Keycode::Kp3 => Some('3'),
                    Keycode::Num4 | Keycode::Kp4 => Some('4'),
                    Keycode::Num5 | Keycode::Kp5 => Some('5'),
                    Keycode::Num6 | Keycode::Kp6 => Some('6'),
                    Keycode::Num7 | Keycode::Kp7 => Some('7'),
                    Keycode::Num8 | Keycode::Kp8 => Some('8'),
                    Keycode::Num9 | Keycode::Kp9 => Some('9'),
                    Keycode::Colon => Some(':'),
                    Keycode::Period | Keycode::KpPeriod => Some('.'),
                    Keycode::A => Some('A'),
                    Keycode::B => Some('B'),
                    Keycode::C => Some('C'),
                    Keycode::D => Some('D'),
                    Keycode::E => Some('E'),
                    Keycode::F => Some('F'),
                    Keycode::Backspace | Keycode::Delete => {
                        self.address.truncate(self.address.len() - 1);
                        None
                    }
                    _ => None,
                } {
                    self.address.push(s);
                    println!("{}", s);
                }
            }
            _ => {}
        }

        Ok(None)
    }

    fn render(&self, canvas: &mut Canvas<Window>, _mouse_state: MouseState) -> Result<(), String> {
        canvas.set_draw_color(WHITE);
        canvas.fill_rect(None)?;
        for button in &self.buttons {
            button.render(canvas)?;
        }
        if let None = self.host_socket {
            self.host_button.render(canvas)?;
        }
        self.connect_button.render(canvas)
    }

    fn do_loop(&mut self) -> Result<Option<Box<Panel>>, String> {
        if let Some(host_socket) = &self.host_socket {
            if let Some(sock) = wait_client(&host_socket) {
                return Ok(Some(Box::new(CreationPanel::new(Some((sock, true))))));
            }
        }
        Ok(None)
    }
}
