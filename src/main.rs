mod boat;
mod direction;
mod elem_view;
mod game;
mod game_view;
mod network;
mod player;
mod view;

use crate::boat::{Boat, Class};
use crate::direction::Direction;
use crate::game::{Game, GameType};
use crate::network::wait_client;
use crate::player::Player;
use std::net::TcpStream;
use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub const NB: i32 = 12;

fn create_game_type() -> Result<GameType, String> {
    for s in std::env::args() {
        if s == "host" {
            return match wait_client() {
                Some(stream) => Ok(GameType::Network {
                    game: create_game(),
                    player: AtomicBool::new(true),
                    socket: stream,
                }),
                None => Err(String::from("Unable to find client")),
            };
        }

        if let Ok(stream) = TcpStream::connect((s.as_str(), 8080)) {
            println!("Successful connexion to {}", s);
            return Ok(GameType::Network {
                game: create_game(),
                player: AtomicBool::new(false),
                socket: stream,
            });
        }
    }

    println!("Ai game");
    Ok(GameType::Ai {
        game: create_game(),
        opponent: create_game(),
    })
}

fn create_game() -> Game {
    let carrier = Boat::new(Class::Carrier, (1, 0), Direction::Right);
    let battleship = Boat::new(Class::Battleship, (0, 2), Direction::Up);
    let cruiser = Boat::new(Class::Cruiser, (11, 1), Direction::Left);
    let submarine = Boat::new(Class::Submarine, (10, 10), Direction::Down);
    let destroyer = Boat::new(Class::Destroyer, (5, 5), Direction::Left);

    let player = Player::new(
        String::from("Pierre"),
        vec![carrier, battleship, cruiser, submarine, destroyer],
    );

    Game::new(NB as usize, NB as usize, player)
}

fn main() {
    let gt = create_game_type();
    let mutex_game = match gt {
        Err(err) => panic!("{:?}", err),
        Ok(game) => {
            println!("there is a game");
            Arc::new(Mutex::new(game))
        }
    };

    let work = Arc::new(AtomicBool::new(true));

    let m = mutex_game.clone();
    let w = work.clone();
    let child1 = thread::spawn(move || {
        view::view_thread(m, w);
    });

    let m = mutex_game.clone();
    let w = work.clone();
    let child2 = thread::spawn(move || {
        network::network_thread(m, w);
    });

    child1.join().unwrap();
    child2.join().unwrap();
}

pub fn quit(mutex: &mut Arc<Mutex<GameType>>, work: &mut Arc<AtomicBool>) {
    println!("quit");
    work.store(false, Ordering::Relaxed);
    if let GameType::Network { socket, .. } = mutex.lock().unwrap().deref_mut() {
        socket.shutdown(std::net::Shutdown::Both).unwrap();
    }
    std::process::exit(0);
}
