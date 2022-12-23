#[derive(Copy, Clone)]
pub enum Direction {
    N,
    E,
    S,
    W,
    NE,
    NW,
    SE,
    SW,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn translate_new(&self, direction: Direction) -> Self {
        match direction {
            Direction::N => Self::new(self.x, self.y - 1),
            Direction::E => Self::new(self.x + 1, self.y),
            Direction::S => Self::new(self.x, self.y + 1),
            Direction::W => Self::new(self.x - 1, self.y),
            Direction::NE => Self::new(self.x + 1, self.y - 1),
            Direction::NW => Self::new(self.x - 1, self.y - 1),
            Direction::SE => Self::new(self.x + 1, self.y + 1),
            Direction::SW => Self::new(self.x - 1, self.y + 1),
        }
    }
}
