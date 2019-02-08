mod boat;
mod direction;
mod game;
mod network;
mod player;
mod view;

use crate::boat::{Boat, Class};
use crate::direction::Direction;
use crate::game::{Game, GameType};
use crate::player::Player;
use std::net::{TcpListener, TcpStream};
use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub const NB: i32 = 12;

fn wait_client() -> Option<TcpStream> {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    match listener.accept() {
        Ok((client, addr)) => {
            println!("A client was found : {}", addr);
            return Some(client);
        }
        _ => {}
    }
    None
}

fn create_game_type() -> Result<GameType, String> {
    for s in std::env::args() {
        let tcp_s = TcpStream::connect((s.as_str(), 8080));
        match tcp_s {
            Ok(stream) => {
                println!("Successful connexion to {}", s);
                return Ok(GameType::Network {
                    game: create_game(),
                    player: AtomicBool::new(false),
                    socket: stream,
                });
            }
            Err(st) => {
                println!("{}", st);
                if s == "host" {
                    let tcp_s = wait_client();
                    return match tcp_s {
                        Some(stream) => Ok(GameType::Network {
                            game: create_game(),
                            player: AtomicBool::new(true),
                            socket: stream,
                        }),
                        None => Err(String::from("Unable to find client")),
                    };
                }
            }
        }
    }

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
    match mutex.lock().unwrap().deref_mut() {
        GameType::Network { socket, .. } => {
            socket.shutdown(std::net::Shutdown::Both).unwrap();
        }
        _ => (),
    }
    std::process::exit(0);
}
