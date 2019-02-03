mod boat;
mod direction;
mod game;
mod player;
mod view;

use crate::boat::{Boat, Class};
use crate::direction::Direction;
use crate::game::{Game, GameType};
use crate::player::Player;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn wait_client() -> Option<TcpStream> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
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
                return Ok(GameType::Network(create_game(), stream));
            }
            _ => {
                if s == "host" {
                    let tcp_s = wait_client();
                    return match tcp_s {
                        Some(stream) => Ok(GameType::Network(create_game(), stream)),
                        None => Err(String::from("Unable to find client")),
                    };
                }
            }
        }
    }

    Err(String::from("nope"))
}

fn create_game() -> Game {
    let carrier = Boat::new(Class::Carrier, (0, 0), Direction::Right);
    let battleship = Boat::new(Class::Battleship, (0, 1), Direction::Up);
    let cruiser = Boat::new(Class::Cruiser, (11, 1), Direction::Left);
    let submarine = Boat::new(Class::Submarine, (11, 11), Direction::Down);
    let destroyer = Boat::new(Class::Destroyer, (5, 5), Direction::Left);

    let player = Player::new(
        String::from("Pierre"),
        vec![carrier, battleship, cruiser, submarine, destroyer],
    );

    Game::new(12, 12, player)
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

    let mut1 = mutex_game.clone();

    thread::spawn(move || {
        view::view_thread(mut1);
    });
}
