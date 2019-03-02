// menu panel
use crate::{
    utils::*,
    view::{
        buttons::Button,
        connection::ConnectPanel,
        creation::CreationPanel,
        panel::{Panel, QUIT_COLOR, TEXT_COLOR},
        HEIGHT, WIDTH,
    },
};

pub struct MenuPanel {
    buttons: Vec<Button>,
}

impl MenuPanel {
    pub fn new(panel: *mut Option<Box<Panel>>) -> Self {
        let n = 3;
        let width = 200;
        let height = 100;
        let v_space = (HEIGHT - n * height) / (n + 1);
        let h_space = (WIDTH - width) / 2;

        MenuPanel {
            buttons: vec![
                Button::new(
                    h_space,
                    v_space,
                    width,
                    height,
                    GREEN,
                    "Multiplayer game".to_string(),
                    TEXT_COLOR,
                    Box::new(move || unsafe {
                        *panel = Some(Box::new(ConnectPanel::new(panel))); true
                    }),
                ),
                Button::new(
                    h_space,
                    2 * v_space + height,
                    width,
                    height,
                    BLUE,
                    "AI game".to_string(),
                    TEXT_COLOR,
                    Box::new(move || unsafe {
                        *panel = Some(Box::new(CreationPanel::new(panel, None))); true
                    }),
                ),
                Button::new(
                    h_space,
                    3 * v_space + 2 * height,
                    width,
                    height,
                    QUIT_COLOR,
                    "Quit".to_string(),
                    TEXT_COLOR,
                    Box::new(|| std::process::exit(1)),
                ),
            ],
        }
    }
}

impl Panel for MenuPanel {
    fn button_vec_mut(&mut self) -> &mut Vec<Button> {
        &mut self.buttons
    }

    fn button_vec(&self) -> &Vec<Button> {
        &self.buttons
    }
}
