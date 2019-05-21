use crate::model::{
    boat::{Boat, Class},
    direction::Direction,
    game::{Game, GameType},
    player::Player,
};
use crate::utils::*;
use crate::view::{
    buttons::Button,
    game::{GamePanel, BOARD_WIDTH, SIZE},
    game_renderer::GameRenderer,
    menu::MenuPanel,
    panel::{Panel, TEXT_COLOR},
    NB, WIDTH,
};
use sdl2::{
    event::{
        Event,
        Event::{KeyUp, MouseButtonDown, MouseButtonUp},
    },
    keyboard::Keycode,
    mouse::{MouseButton, MouseState},
    render::Canvas,
    video::Window,
};
use std::{mem::swap, net::TcpStream};

const OFFSET_BOARD_X: i32 = (WIDTH - BOARD_WIDTH) / 2;
const OFFSET_BOARD_Y: i32 = 0;
const OFFSET_PB_X: i32 = WIDTH - 6 * SIZE;
const OFFSET_PB_Y: i32 = SIZE;

macro_rules! get_order {
    ($x : expr) => {
        match &$x.class {
            Class::Carrier => 0,
            Class::Battleship => 1,
            Class::Cruiser => 2,
            Class::Submarine => 3,
            Class::Destroyer => 4,
        }
    };
}

// game's creation panel
pub struct CreationPanel {
    buttons: Vec<Button>,
    board: Vec<Vec<Option<u8>>>,
    player: Player,
    pending_boats: Vec<Option<Boat>>, // boats that haven't been placed yet
    network: Option<(TcpStream, bool)>,
    start_button: Button,
    selected: Option<Boat>,
    panel: *mut Option<Box<Panel>>,
}

impl CreationPanel {
    pub fn new(panel: *mut Option<Box<Panel>>, network: Option<(TcpStream, bool)>) -> Self {
        let pending_boats = vec![
            Some(Boat::new(Class::Carrier, (0, 0), Direction::Right)),
            Some(Boat::new(Class::Battleship, (0, 0), Direction::Right)),
            Some(Boat::new(Class::Cruiser, (0, 0), Direction::Right)),
            Some(Boat::new(Class::Submarine, (0, 0), Direction::Right)),
            Some(Boat::new(Class::Destroyer, (0, 0), Direction::Right)),
        ];
        CreationPanel {
            buttons: vec![],
            start_button: Button::new(
                0,
                0,
                100,
                100,
                YELLOW,
                "Start".to_string(),
                TEXT_COLOR,
                Box::new(|| false),
            ),
            board: vec![vec![None; NB as usize]; NB as usize],
            player: Player::new(Vec::new()),
            pending_boats,
            network,
            selected: None,
            panel,
        }
    }

    fn into_game_type(&mut self) -> Result<GameType, String> {
        let mut player2 = Player::new(vec![]);
        swap(&mut player2, &mut self.player);

        Ok(match &self.network {
            Some((socket, player)) => GameType::Network {
                game: Game::new(NB as usize, NB as usize, player2)?,
                player: *player,
                socket: socket.try_clone().unwrap(),
            },
            None => GameType::Ai {
                game: Game::new(NB as usize, NB as usize, player2)?,
                opponent: Game::create_ai_game()?,
                player: true,
            },
        })
    }
}

impl Panel for CreationPanel {
    fn button_vec_mut(&mut self) -> &mut Vec<Button> {
        &mut self.buttons
    }

    fn button_vec(&self) -> &Vec<Button> {
        &self.buttons
    }

    fn manage_event(&mut self, event: Event) -> Result<bool, String> {
        match event {
            MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                // click on the start button
                if self.start_button.contains_point((x, y)) {
                    unsafe {
                        *self.panel = Some(match self.into_game_type() {
                            Ok(game) => Box::new(GamePanel::new(self.panel, game)),
                            Err(err) => {
                                println!("{}", err);
                                Box::new(MenuPanel::new(self.panel))
                            }
                        })
                    }
                    return Ok(true);
                }

                // drop a boat
                let mut boat: Option<Boat> = None;
                swap(&mut boat, &mut self.selected);
                if let Some(mut boat) = boat {
                    println!("drop the boat");
                    let xx = ((x - OFFSET_BOARD_X) / SIZE) as usize;
                    let yy = ((y - OFFSET_BOARD_Y) / SIZE) as usize;
                    let dx = boat.direction.dx() as usize;
                    let dy = boat.direction.dy() as usize;
                    let life = boat.max_life() as usize;
                    println!("{},{}", xx, yy);

                    let mut valid = in_board!(xx, yy, NB, NB)
                    // the head of the boat is in the board
                        && in_board!(
                            xx + dx * (life - 1),
                            yy + dy * (life - 1),
                            NB,
                            NB
                        );
                    // the tail of the boat is in the board

                    if valid {
                        println!("fully in board, let's see if the position is empty");
                        for i in 0..life {
                            valid &= self.board[xx + i * dx][yy + i * dy] == None;
                        }
                    } else {
                        println!("Not even fully in the board");
                    }
                    if valid {
                        println!(
                            "valid drop of the boat xx:{}, yy:{}, life:{}, dx:{}, dy:{}",
                            xx, yy, life, dx, dy
                        );
                        boat.position = (xx as u8, yy as u8);
                        self.player.boats.push(boat);
                        let pos = (self.player.boats.len() - 1) as u8;
                        for i in 0..life {
                            self.board[xx + dx * i][yy + i * dy] = Some(pos);
                        }
                    } else {
                        println!("invalid drop of the boat");
                        boat.direction = Direction::Right;
                        let i = get_order!(boat);
                        self.pending_boats[i] = Some(boat);
                    }
                }
            }
            KeyUp {
                keycode: Some(Keycode::R),
                ..
            }
            | MouseButtonUp {
                mouse_btn: MouseButton::Right,
                ..
            } => {
                if let Some(boat) = &mut self.selected {
                    boat.direction = boat.direction.rotate();
                }
            }
            MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                println!("mouse down");
                if in_board!(
                    x,
                    y,
                    5 * SIZE,
                    5 * SIZE - SIZE / 2,
                    OFFSET_PB_X,
                    OFFSET_PB_Y
                ) {
                    println!("trying to take a boat");
                    let i = y / SIZE - 1;
                    let mut boat: Option<Boat> = None;
                    swap(&mut self.pending_boats[i as usize], &mut boat);
                    if let Some(_) = &boat {
                        self.selected = boat;
                    }
                }
            }

            _ => (),
        }
        Ok(false)
    }

    fn render(&self, canvas: &mut Canvas<Window>, mouse_state: MouseState) -> Result<(), String> {
        canvas.set_draw_color(WHITE);
        canvas.fill_rect(None)?;
        for button in &self.buttons {
            button.render(canvas)?;
        }

        canvas.render_board_boat(
            &self.player,
            (OFFSET_BOARD_X, OFFSET_BOARD_Y),
            (NB, NB),
            SIZE,
        )?;
        self.start_button.render(canvas)?;

        if let Some(boat) = &self.selected {
            canvas.render_boat_at(&boat, (mouse_state.x(), mouse_state.y()))?;
        }

        for i in 0..self.pending_boats.len() {
            if let Some(boat) = &self.pending_boats[i] {
                canvas.render_boat_at(
                    boat,
                    (
                        OFFSET_PB_X + SIZE / 2,
                        OFFSET_PB_Y + SIZE / 2 + i as i32 * SIZE,
                    ),
                )?;
            }
        }

        canvas.render_grid((OFFSET_PB_X, OFFSET_PB_Y), (5, 5), SIZE)?;
        Ok(())
    }
}
