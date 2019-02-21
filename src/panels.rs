use crate::boat::Boat;
use crate::game::{Game, GameType};
use crate::game_view::GameRenderer;
use crate::player::Player;
use crate::utils::*;
use crate::view::{HEIGHT, OFFSET_X, SIZE, WIDTH};
use crate::NB;
use sdl2::event::{Event, Event::MouseButtonUp};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::net::TcpStream;

const QUIT_COLOR: Color = RED;
const TEXT_COLOR: Color = BLACK;

pub trait Panel {
    fn button_vec_mut(&mut self) -> &mut Vec<Button>;
    fn button_vec(&self) -> &Vec<Button>;

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(WHITE);
        canvas.fill_rect(None)?;
        for button in self.button_vec() {
            button.render(canvas)?;
        }
        Ok(())
    }

    // the Ok part is true to continue the main loop and false otherwise
    fn manage_event(&mut self, event: Event) -> Result<bool, String> {
        if let MouseButtonUp {
            mouse_btn: MouseButton::Left,
            x,
            y,
            ..
        } = event
        {
            for button in self.button_vec_mut() {
                if button.contains_point((x, y)) {
                    button.execute();
                }
            }
        }
        Ok(false)
    }

    // called each loop turn
    // does nothing by default
    // the Ok part is true to continue the main loop and false otherwise
    fn do_loop(&mut self) -> Result<bool, String> {
        Ok(false)
    }
}

// menu panel
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
                    Box::new(move || unsafe { *panel = Some(Box::new(ConnectPanel::new(panel))) }),
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
                        *panel = Some(Box::new(CreationPanel::new(panel, None)))
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

// game panel
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

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
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
            if x >= OFFSET_X && y >= 0 && x < WIDTH && y < HEIGHT {
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

// multiplayer connection panel
pub struct ConnectPanel {
    buttons: Vec<Button>,
}

impl ConnectPanel {
    fn new(panel: *mut Option<Box<Panel>>) -> Self {
        ConnectPanel { buttons: vec![] }
    }
}

impl Panel for ConnectPanel {
    fn button_vec_mut(&mut self) -> &mut Vec<Button> {
        &mut self.buttons
    }

    fn button_vec(&self) -> &Vec<Button> {
        &self.buttons
    }
}

// game's creation panel
pub struct CreationPanel {
    buttons: Vec<Button>,
    board: Vec<Vec<Option<u8>>>,
    player: Player,
    pending_boats: Vec<Boat>,
    network: Option<(TcpStream, bool)>,
    start_button: Button,
    panel: *mut Option<Box<Panel>>,
}

impl CreationPanel {
    fn new(panel: *mut Option<Box<Panel>>, network: Option<(TcpStream, bool)>) -> Self {
        let boat_list: Vec<Boat> = vec![];
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
                Box::new(|| {}),
            ),
            board: vec![vec![None; NB as usize]; NB as usize],
            player: Player::new(Vec::new()),
            pending_boats: boat_list,
            network,
            panel,
        }
    }

    fn into_game_type(&mut self) -> Result<GameType, String> {
        let mut player2 = Player::new(vec![]);
        std::mem::swap(&mut player2, &mut self.player);

        Ok(match &self.network {
            Some((socket, player)) => GameType::Network {
                game: Game::create_ai_game()?, //Game::new(NB as usize, NB as usize, player2),
                player: *player,
                socket: socket.try_clone().unwrap(),
            },
            None => GameType::Ai {
                game: Game::create_ai_game()?, //Game::new(NB as usize, NB as usize, player2),
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
        if let MouseButtonUp {
            mouse_btn: MouseButton::Left,
            x,
            y,
            ..
        } = event
        {
            if self.start_button.contains_point((x, y)) {
                unsafe {
                    *self.panel = Some(match self.into_game_type() {
                        Ok(game) => Box::new(GamePanel::new(self.panel, game)),
                        Err(err) => {
                            println!("{}", err);
                            Box::new(MenuPanel::new(self.panel))
                        }
                    })
                };
            }
        }
        Ok(false)
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(WHITE);
        canvas.fill_rect(None)?;
        for button in &self.buttons {
            button.render(canvas)?;
        }
        self.start_button.render(canvas)?;
        Ok(())
    }
}

// end of game panel
pub struct EndGamePanel {
    buttons: Vec<Button>,
}

impl EndGamePanel {
    pub fn new(panel: *mut Option<Box<Panel>>, win: bool) -> Self {
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
                    Box::new(|| ()),
                ),
                Button::new(
                    h_space,
                    2 * v_space + height,
                    width,
                    height,
                    BLUE,
                    "Back to menu".to_string(),
                    TEXT_COLOR,
                    Box::new(move || unsafe { *panel = Some(Box::new(MenuPanel::new(panel))) }),
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

pub struct Button {
    background: Color,
    position: Rect,
    text: String,
    text_color: Color,
    action: Box<FnMut() -> ()>,
}

impl Button {
    fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        background: Color,
        text: String,
        text_color: Color,
        action: Box<FnMut() -> ()>,
    ) -> Button {
        Button {
            position: Rect::new(x, y, w as u32, h as u32),
            background,
            text,
            text_color,
            action,
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.background);
        canvas.fill_rect(self.position)?;

        Ok(())
    }

    fn contains_point<P: Into<(i32, i32)>>(&self, point: P) -> bool {
        self.position.contains_point(point)
    }

    fn execute(&mut self) {
        (self.action)();
    }
}
