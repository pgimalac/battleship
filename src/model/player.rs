use crate::model::boat::Boat;

// to change ?
// add a username ? any more informations ?
#[derive(Debug)]
pub struct Player {
    pub boats: Vec<Boat>,
}

impl Player {
    pub fn new(boats: Vec<Boat>) -> Self {
        Player { boats }
    }

    pub fn is_dead(&self) -> bool {
        for boat in &self.boats {
            if !boat.is_dead() {
                return false;
            }
        }
        true
    }
}
