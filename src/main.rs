#[macro_use]
mod utils;
mod model;
mod network;
mod view;

pub const NB: i32 = 12;

fn main() -> Result<(), String> {
    view::run()
}
