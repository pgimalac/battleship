use crate::boat::Boat;
use crate::game;
use crate::game::{Game, GameType};
use crate::utils::*;
use crate::view::{BOARD_WIDTH, OFFSET_X, SIZE, WIDTH};
use crate::NB;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

const HEALTHY_BOAT: Color = BLUE;
const WOUNDED_BOAT: Color = RED;

pub trait GameRenderer {
    fn render_game_type(&mut self, game_type: &GameType) -> Result<(), String>;
    fn render_game(&mut self, game: &Game) -> Result<(), String>;
    fn render_boat(&mut self, boat: &Boat) -> Result<(), String>;
}

impl GameRenderer for Canvas<Window> {
    fn render_game_type(&mut self, game_type: &GameType) -> Result<(), String> {
        self.set_draw_color(WHITE);
        self.fill_rect(None).unwrap();

        self.set_draw_color(BLACK);
        for i in 0..NB + 1 {
            self.draw_line((0, i * SIZE), (BOARD_WIDTH, i * SIZE))?;
            self.draw_line((OFFSET_X, i * SIZE), (WIDTH, i * SIZE))?;

            self.draw_line((i * SIZE, 0), (i * SIZE, BOARD_WIDTH))?;
            self.draw_line((OFFSET_X + i * SIZE, 0), (OFFSET_X + i * SIZE, BOARD_WIDTH))?;
        }
        self.render_game(game!(game_type))
    }

    fn render_game(&mut self, game: &Game) -> Result<(), String> {
        for boat in &game.player.boats {
            self.render_boat(&boat)?;
        }

        for i in 0..game.shot_boats.len() {
            for j in 0..game.shot_boats[i].len() {
                if let Some(b) = game.shot_boats[i][j] {
                    fill_circle(
                        self,
                        if b { WOUNDED_BOAT } else { HEALTHY_BOAT },
                        OFFSET_X + SIZE / 2 + i as i32 * SIZE,
                        SIZE / 2 + j as i32 * SIZE,
                        SIZE / 3,
                    )?
                }
            }
        }

        Ok(())
    }

    fn render_boat(&mut self, boat: &Boat) -> Result<(), String> {
        let dx = boat.direction.dx() as i32;
        let dy = boat.direction.dy() as i32;
        let mut x = boat.position.0 as i32;
        let mut y = boat.position.1 as i32;

        for l in &boat.detailed_life {
            fill_circle(
                self,
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
