#[macro_use] mod utils;
mod game_view;
mod boat;
mod direction;
mod game;
mod network;
mod panels;
mod player;
mod view;

pub const NB: i32 = 12;

fn main() {
    if let Err(err) = view::run() {
        panic!("{}", err);
    }
}
