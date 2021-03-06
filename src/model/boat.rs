type Position = (u8, u8);
use crate::model::direction::Direction;

#[derive(Copy, Clone, Debug)]
pub enum Class {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

impl Class {
    pub fn max_life(&self) -> u8 {
        match self {
            Class::Carrier => 5,
            Class::Battleship => 4,
            Class::Cruiser => 3,
            Class::Submarine => 3,
            Class::Destroyer => 2,
        }
    }
}

#[derive(Debug)]
pub struct Boat {
    pub class: Class,
    pub position: Position,
    pub direction: Direction,
    pub detailed_life: Vec<bool>,
}

impl Boat {
    pub fn is_dead(&self) -> bool {
        for b in &self.detailed_life {
            if *b {
                return false;
            }
        }
        true
    }

    pub fn new(class: Class, position: Position, direction: Direction) -> Self {
        Boat {
            class,
            position,
            direction,
            detailed_life: vec![true; class.max_life() as usize],
        }
    }

    pub fn max_life(&self) -> u8 {
        self.class.max_life()
    }

    pub fn shoot(&mut self, position: Position) -> bool {
        let n: i8;

        if self.direction.dx() == 0 {
            if self.position.0 == position.0 && self.direction.dy() != 0 {
                n = (position.1 as i8 - self.position.1 as i8) / self.direction.dy();
            } else {
                return false;
            }
        } else if self.direction.dy() == 0 {
            if self.position.1 == position.1 {
                n = (position.0 as i8 - self.position.0 as i8) / self.direction.dx();
            } else {
                return false;
            }
        } else {
            return false;
        }

        if n < 0 || n >= self.max_life() as i8 || !self.detailed_life[n as usize] {
            return false;
        }

        self.detailed_life[n as usize] = false;
        return true;
    }
}
