use crate::model::{
    boat::{Boat, Class},
    direction::Direction,
    player::Player,
};
use crate::network::{ATTACK, CONFIRM};
use crate::NB;
use rand::Rng;
use std::io::Write;

#[macro_export]
macro_rules! game {
    ($x : expr) => {
        match $x {
            GameType::Network { game, .. } => game,
            GameType::Ai { game, .. } => game,
        }
    };
}

#[derive(Debug)]
pub struct Game {
    pub board_boats: Vec<Vec<Option<u8>>>,
    pub shot_boats: Vec<Vec<Option<bool>>>,
    pub player: Player,
}

impl Game {
    pub fn shot(&self, p: (u8, u8)) -> bool {
        self.shot_boats[p.0 as usize][p.1 as usize] != None
    }

    pub fn create_ai_game() -> Result<Self, String> {
        let carrier = Boat::new(Class::Carrier, (1, 0), Direction::Right);
        let battleship = Boat::new(Class::Battleship, (0, 2), Direction::Up);
        let cruiser = Boat::new(Class::Cruiser, (11, 1), Direction::Left);
        let submarine = Boat::new(Class::Submarine, (10, 10), Direction::Down);
        let destroyer = Boat::new(Class::Destroyer, (5, 5), Direction::Left);

        let player = Player::new(vec![carrier, battleship, cruiser, submarine, destroyer]);

        Game::new(NB as usize, NB as usize, player)
    }

    pub fn confirm_attack(&mut self, p: (u8, u8), b: bool) {
        self.shot_boats[p.0 as usize][p.1 as usize] = Some(b);
    }

    pub fn opponent_attack(&mut self, p: (u8, u8)) -> bool {
        println!("game : opponent shot ({},{})", p.0, p.1);
        match &self.board_boats[p.0 as usize][p.1 as usize] {
            None => false,
            Some(i) => self.player.boats[*i as usize].shoot(p),
        }
    }

    pub fn new(size_x: usize, size_y: usize, player: Player) -> Result<Self, String> {
        let mut board_boats: Vec<Vec<Option<u8>>> = vec![vec![None; size_y]; size_x];

        let mut i = 0;
        for boat in &player.boats {
            let d = boat.direction.delta();
            let mut x = boat.position.0 as i8;
            let mut y = boat.position.1 as i8;

            if !in_board!(x, y, board_boats.len(), board_boats[0].len())
                || !in_board!(
                    x as i8 + d.0 * (boat.max_life() - 1) as i8,
                    y as i8 + d.1 * (boat.max_life() - 1) as i8,
                    board_boats.len(),
                    board_boats[0].len()
                )
            {
                return Err(format!("Wrong boat position {:?}", boat));
            }
            for _ in 0..boat.max_life() {
                match &board_boats[x as usize][y as usize] {
                    Some(b) => {
                        return Err(format!(
                            "Boat overleap : {:?} and {:?}",
                            player.boats[*b as usize], boat
                        ))
                    }
                    None => board_boats[x as usize][y as usize] = Some(i),
                }
                x += d.0;
                y += d.1;
            }
            i += 1;
        }

        Ok(Game {
            board_boats,
            shot_boats: vec![vec![None; size_y]; size_x],
            player,
        })
    }

    // to improve ?
    // not efficient when the grid is almost filled
    // and very stupid
    pub fn get_auto_position(&self) -> (u8, u8) {
        let nb = NB as u8;
        let mut p: (u8, u8);
        loop {
            p = (
                rand::thread_rng().gen_range(0, nb),
                rand::thread_rng().gen_range(0, nb),
            );
            if self.shot_boats[p.0 as usize][p.1 as usize] == None {
                println!("auto_attack on {:?}", p);
                return p;
            }
        }
    }
}

#[derive(Debug)]
pub enum GameType {
    Network {
        game: Game,                  // the game of the main player
        player: bool,                // true for player 1, false otherwise
        socket: std::net::TcpStream, // the socket to the other player
    },
    Ai {
        game: Game, // the game of the main player
        player: bool,
        opponent: Game, // the game of the AI
    },
}

impl GameType {
    // called when the opponent attacks a position
    pub fn opponent_attack(&mut self, p: (u8, u8)) -> Result<(), String> {
        println!("game_type : opponent_attack ({},{})", p.0, p.1);
        match self {
            GameType::Network {
                game,
                socket,
                player,
            } => {
                let b = game.opponent_attack(p);
                *player = true;
                println!("Message sent : confirm ({}, {}) as {}", p.0, p.1, b);
                result_map!(
                    socket.write(&[CONFIRM, p.0, p.1, b as u8]),
                    |_| (),
                    |x: std::io::Error| x.to_string()
                )
            }
            GameType::Ai { game, .. } => {
                game.opponent_attack(p);
                Ok(())
            }
        }
    }

    // called to confirm the main player attack result
    pub fn confirm_attack(&mut self, p: (u8, u8), b: bool) -> Result<(), String> {
        println!("confirm_attack");
        match self {
            GameType::Network { game, player, .. } => {
                game.confirm_attack(p, b);
                *player = false;
                Ok(())
            }
            GameType::Ai {
                game,
                player,
                opponent,
            } => {
                if *player {
                    game.confirm_attack(p, b);
                    *player = false;
                    if self.is_over() == None {
                        self.auto_attack()
                    } else {
                        Ok(())
                    }
                } else {
                    opponent.confirm_attack(p, b);
                    *player = true;
                    Ok(())
                }
            }
        }
    }

    pub fn auto_attack(&mut self) -> Result<(), String> {
        println!("auto_play");
        if self.is_over() != None {
            return Err("Game already over".to_string());
        }
        let game = match self {
            GameType::Network { player: true, .. } => {
                return Err("Not your turn, invalid action".to_string())
            }
            GameType::Network { game, .. } => game,
            GameType::Ai {
                game,
                opponent,
                player,
            } => {
                if *player {
                    game
                } else {
                    opponent
                }
            }
        };
        let p = game.get_auto_position();
        self.attack(p)
    }

    pub fn is_over(&self) -> Option<bool> {
        match self {
            // TODO
            GameType::Network { player: false, .. } => None,
            GameType::Network { game, .. } => {
                if game.player.is_dead() {
                    Some(false)
                } else {
                    None
                }
            }
            GameType::Ai { game, opponent, .. } => {
                if game.player.is_dead() {
                    Some(false)
                } else if opponent.player.is_dead() {
                    Some(true)
                } else {
                    None
                }
            }
        }
    }

    // called to attack a position
    pub fn attack(&mut self, p: (u8, u8)) -> Result<(), String> {
        println!("attack ({};{})", p.0, p.1);
        match self {
            GameType::Network {
                game,
                socket,
                player,
            } => {
                /* TODO
                    Record the attack position to avoid multiple attacks
                    before the opponent answers
                */
                if *player && game.shot_boats[p.0 as usize][p.1 as usize] == None {
                    println!("message sent : attack ({};{})", p.0, p.1);
                    result_map!(
                        socket.write(&[ATTACK, p.0, p.1]),
                        |_| (),
                        |x: std::io::Error| x.to_string()
                    )
                } else {
                    Ok(())
                }
            }
            GameType::Ai {
                game,
                opponent,
                player,
            } => {
                let b = if *player {
                    if !game.shot(p) {
                        opponent
                    } else {
                        return Ok(());
                    }
                } else {
                    if !opponent.shot(p) {
                        game
                    } else {
                        return Ok(());
                    }
                }
                .opponent_attack(p);
                self.confirm_attack(p, b)
            }
        }
    }
}
