use crate::network::ATTACK;
use crate::network::CONFIRM;
use crate::player::Player;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub struct Game {
    pub board_boats: Vec<Vec<Option<u8>>>,
    pub shot_boats: Vec<Vec<Option<bool>>>,
    pub player: Player,
}

fn in_board(size_x: u8, size_y: u8, x: i8, y: i8) -> bool {
    x >= 0 && y >= 0 && x < size_x as i8 && y < size_y as i8
}

impl Game {
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

    pub fn new(size_x: usize, size_y: usize, player: Player) -> Game {
        let mut board_boats: Vec<Vec<Option<u8>>> = vec![vec![None; size_y]; size_x];

        let mut i = 0;
        for boat in &player.boats {
            let d = boat.direction.delta();
            let mut x = boat.position.0 as i8;
            let mut y = boat.position.1 as i8;

            if !in_board(board_boats.len() as u8, board_boats[0].len() as u8, x, y)
                || !in_board(
                    board_boats.len() as u8,
                    board_boats[0].len() as u8,
                    x + d.0 * boat.max_life() as i8,
                    y + d.1 * boat.max_life() as i8,
                )
            {
                panic!("Wrong boat position {:?}", boat);
            }
            for _ in 0..boat.max_life() {
                match &board_boats[x as usize][y as usize] {
                    Some(b) => panic!(
                        "Boat overleap : {:?} and {:?}",
                        player.boats[*b as usize], boat
                    ),
                    None => board_boats[x as usize][y as usize] = Some(i),
                }
                x += d.0;
                y += d.1;
            }
            i += 1;
        }

        Game {
            board_boats,
            shot_boats: vec![vec![None; size_y]; size_x],
            player,
        }
    }
}

#[derive(Debug)]
pub enum GameType {
    Network {
        game: Game,                  // the game of the main player
        player: AtomicBool,          // true for player 1, false otherwise
        socket: std::net::TcpStream, // the socket to the other player
    },
    Ai {
        game: Game,     // the game of the main player
        opponent: Game, // the game of the AI
    },
}

impl GameType {
    pub fn get_mut_main_game(&mut self) -> &mut Game {
        match self {
            GameType::Network { game, .. } => game,
            GameType::Ai { game, .. } => game,
        }
    }

    pub fn get_main_game(&self) -> &Game {
        match self {
            GameType::Network { game, .. } => game,
            GameType::Ai { game, .. } => game,
        }
    }

    // called when the opponent attacks a position
    pub fn opponent_attack(&mut self, p: (u8, u8)) {
        println!("game_type : opponent_attack ({},{})", p.0, p.1);
        match self {
            GameType::Network {
                game,
                socket,
                player,
            } => {
                let b = game.opponent_attack(p);
                player.store(true, Ordering::Relaxed);
                socket
                    .write(&[CONFIRM, p.0, p.1, if b { 1 } else { 0 }])
                    .unwrap();
                println!("Message sent : confirm ({}, {}) as {}", p.0, p.1, b);
            }
            GameType::Ai { game, .. } => {
                game.opponent_attack(p);
            }
        }
    }

    // called to confirm the main player attack result
    pub fn confirm_attack(&mut self, p: (u8, u8), b: bool) {
        match self {
            GameType::Network { game, player, .. } => {
                game.confirm_attack(p, b);
                player.store(false, Ordering::Relaxed);
            }
            GameType::Ai { game, .. } => {
                game.confirm_attack(p, b);
                //TODO : ai plays
            }
        }
    }

    // called to attack a position
    pub fn attack(&mut self, p: (u8, u8)) {
        println!("attack ({};{})", p.0, p.1);
        match self {
            GameType::Network {
                game,
                socket,
                player,
            } => {
                if player.load(Ordering::Relaxed)
                    && game.shot_boats[p.0 as usize][p.1 as usize] == None
                {
                    socket.write(&[ATTACK, p.0, p.1]).unwrap();
                    println!("message sent : attack ({};{})", p.0, p.1);
                }
            }
            GameType::Ai { game, opponent } => {
                if game.shot_boats[p.0 as usize][p.1 as usize] == None {
                    game.confirm_attack(p, opponent.opponent_attack(p))
                }
            }
        }
    }
}
