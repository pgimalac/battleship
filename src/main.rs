mod boat;
mod direction;
mod game;
mod player;

use crate::boat::Boat;
use crate::boat::Class;
use crate::direction::Direction;
use crate::game::Game;
use crate::player::Player;
use std::rc::Rc;

fn main() {
    let carrier = Boat::new(Class::Carrier, (0, 0), Direction::Right);
    let battleship = Boat::new(Class::Battleship, (0, 1), Direction::Up);
    let cruiser = Boat::new(Class::Cruiser, (11, 1), Direction::Left);
    let submarine = Boat::new(Class::Submarine, (11, 11), Direction::Down);
    let destroyer = Boat::new(Class::Destroyer, (5, 5), Direction::Left);

    let player = Player::new(
        String::from("Pierre"),
        vec![
            Rc::new(carrier),
            Rc::new(battleship),
            Rc::new(cruiser),
            Rc::new(submarine),
            Rc::new(destroyer),
        ],
    );

    let game = Game::new(12, 12, player);
    println!("{:#?}", game);
}
