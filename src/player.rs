use crate::boat::Boat;
use std::rc::Rc;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub boats: Vec<Rc<Boat>>,
}

impl Player {
    pub fn new(name: String, boats: Vec<Rc<Boat>>) -> Player {
        Player { name, boats }
    }
}
