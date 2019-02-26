use crate::game;
use crate::model::{
    boat::Boat,
    game::{Game, GameType},
    player::Player,
};
use crate::utils::*;
use crate::view::game::{OFFSET_X, SIZE};
use crate::NB;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

const HEALTHY_BOAT: Color = BLUE;
const WOUNDED_BOAT: Color = RED;

pub trait GameRenderer {
    fn render_game_type(&mut self, game_type: &GameType) -> Result<(), String>;

    fn render_game(&mut self, game: &Game) -> Result<(), String> {
        self.render_board_boat(&game.player, (0, 0), (NB, NB), SIZE)?;
        self.render_shot_board(game)
    }

    fn render_board_boat(
        &mut self,
        player: &Player,
        offset: (i32, i32),
        board_size: (i32, i32),
        tile_size: i32,
    ) -> Result<(), String>;

    fn render_shot_board(&mut self, game: &Game) -> Result<(), String>;

    fn render_grid(
        &mut self,
        offset: (i32, i32),
        board_size: (i32, i32),
        tile_size: i32,
    ) -> Result<(), String>;

    fn render_boat_at(&mut self, boat: &Boat, offset: (i32, i32)) -> Result<(), String>;
}

impl GameRenderer for Canvas<Window> {
    fn render_game_type(&mut self, game_type: &GameType) -> Result<(), String> {
        self.set_draw_color(WHITE);
        self.fill_rect(None).unwrap();

        self.render_game(game!(game_type))
    }

    fn render_grid(
        &mut self,
        offset: (i32, i32),
        board_size: (i32, i32),
        tile_size: i32,
    ) -> Result<(), String> {
        self.set_draw_color(BLACK);

        let width = board_size.0 * tile_size;
        let height = board_size.1 * tile_size;
        for i in 0..board_size.0 + 1 {
            self.draw_line(
                (offset.0 + i * tile_size, offset.1),
                (offset.0 + i * tile_size, offset.1 + height),
            )?;
        }

        for i in 0..board_size.1 + 1 {
            self.draw_line(
                (offset.0, offset.1 + i * tile_size),
                (offset.0 + width, offset.1 + i * tile_size),
            )?;
        }

        Ok(())
    }

    fn render_board_boat(
        &mut self,
        player: &Player,
        offset: (i32, i32),
        board_size: (i32, i32),
        tile_size: i32,
    ) -> Result<(), String> {
        self.render_grid(offset, board_size, tile_size)?;
        self.set_draw_color(BLACK);
        for boat in &player.boats {
            self.render_boat_at(
                &boat,
                (
                    offset.0 + tile_size / 2 + boat.position.0 as i32 * tile_size,
                    offset.1 + tile_size / 2 + boat.position.1 as i32 * tile_size,
                ),
            )?;
        }

        Ok(())
    }

    fn render_shot_board(&mut self, game: &Game) -> Result<(), String> {
        self.render_grid((OFFSET_X, 0), (NB, NB), SIZE)?;
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

    fn render_boat_at(&mut self, boat: &Boat, offset: (i32, i32)) -> Result<(), String> {
        let dx = boat.direction.dx() as i32;
        let dy = boat.direction.dy() as i32;
        let mut len = 0;

        for l in &boat.detailed_life {
            fill_circle(
                self,
                if *l { HEALTHY_BOAT } else { WOUNDED_BOAT },
                offset.0 + len * dx,
                offset.1 + len * dy,
                SIZE / 3,
            )?;

            len += SIZE;
        }
        Ok(())
    }
}
