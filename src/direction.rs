#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up = 1,
    Right = 3,
    Down = -1,
    Left = -3,
}

impl Direction {
    pub fn dx(&self) -> i8 {
        (*self as i8) / 3
    }

    pub fn dy(&self) -> i8 {
        (*self as i8) % 3
    }

    pub fn delta(&self) -> (i8, i8) {
        (self.dx(), self.dy())
    }
}
