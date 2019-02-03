use crate::boat::Boat;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub boats: Vec<Boat>,
}

impl Player {
    pub fn new(name: String, boats: Vec<Boat>) -> Player {
        Player { name, boats }
    }
}
