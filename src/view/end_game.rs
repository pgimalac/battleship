// end of game panel
use crate::utils::*;
use crate::view::{
    buttons::Button,
    menu::MenuPanel,
    panel::{Panel, QUIT_COLOR, TEXT_COLOR},
    HEIGHT, WIDTH,
};

pub struct EndGamePanel {
    buttons: Vec<Button>,
}

impl EndGamePanel {
    pub fn new(win: bool) -> Self {
        let n = 3;
        let width = 200;
        let height = 100;
        let v_space = (HEIGHT - n * height) / (n + 1);
        let h_space = (WIDTH - width) / 2;

        EndGamePanel {
            buttons: vec![
                Button::new(
                    h_space,
                    2 * v_space + height,
                    width,
                    height,
                    MAGENTA,
                    if win { "You won !" } else { "You lose !" }.to_string(),
                    TEXT_COLOR,
                    Box::new(|| None),
                ),
                Button::new(
                    h_space,
                    2 * v_space + height,
                    width,
                    height,
                    CYAN,
                    "Back to menu".to_string(),
                    TEXT_COLOR,
                    Box::new(|| Some(Box::new(MenuPanel::new()))),
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

impl Panel for EndGamePanel {
    fn button_vec_mut(&mut self) -> &mut Vec<Button> {
        &mut self.buttons
    }

    fn button_vec(&self) -> &Vec<Button> {
        &self.buttons
    }
}
