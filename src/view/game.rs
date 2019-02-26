// game panel
use crate::model::game::GameType;
use crate::utils::*;
use crate::view::{
    buttons::Button, end_game::EndGamePanel, game_renderer::GameRenderer, panel::Panel,
};
use crate::NB;
use sdl2::{event::Event, mouse::MouseState, render::Canvas, video::Window};

pub const SIZE: i32 = 50;
pub const DELTA: i32 = 2 * SIZE;
pub const BOARD_WIDTH: i32 = NB * SIZE;
pub const OFFSET_X: i32 = BOARD_WIDTH + 100;

pub struct GamePanel {
    buttons: Vec<Button>,
    game: GameType,
    panel: *mut Option<Box<Panel>>,
}

impl GamePanel {
    pub fn new(panel: *mut Option<Box<Panel>>, game: GameType) -> Self {
        GamePanel {
            buttons: vec![],
            game,
            panel,
        }
    }
}

impl Panel for GamePanel {
    fn button_vec_mut(&mut self) -> &mut Vec<Button> {
        &mut self.buttons
    }

    fn button_vec(&self) -> &Vec<Button> {
        &self.buttons
    }

    fn render(&self, canvas: &mut Canvas<Window>, _mouse_state: MouseState) -> Result<(), String> {
        canvas.set_draw_color(WHITE);
        canvas.fill_rect(None)?;
        for button in &self.buttons {
            button.render(canvas)?;
        }
        canvas.render_game_type(&self.game)?;
        Ok(())
    }

    fn manage_event(&mut self, event: Event) -> Result<bool, String> {
        if let Event::MouseButtonUp { x, y, .. } = event {
            println!("Mouse button up");
            if in_board!(x, y, BOARD_WIDTH, BOARD_WIDTH, OFFSET_X, 0) {
                let x = ((x - OFFSET_X) / SIZE) as u8;
                let y = (y / SIZE) as u8;
                self.game.attack((x, y))?;
            }
        }

        Ok(false)
    }

    fn do_loop(&mut self) -> Result<bool, String> {
        if let Some(b) = self.game.is_over() {
            unsafe {
                *self.panel = Some(Box::new(EndGamePanel::new(self.panel, b)));
            }

            Ok(true)
        } else if let GameType::Network { .. } = self.game {
            self.game.check_network()
        } else {
            Ok(false)
        }
    }
}
