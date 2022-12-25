use crate::direction::Direction;

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
    pub fn from_usizes(x: usize, y: usize) -> Self {
        Self {
            x: x as u8,
            y: y as u8,
        }
    }
    pub fn move_in_direction(self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Stay => self,
        }
    }

    pub fn distance(&self, other: &Self) -> u8 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn blow(self, max_x: u8, max_y: u8, wind: Direction) -> Self {
        let mut new_coordinate = self.move_in_direction(&wind);

        if new_coordinate.x == max_x {
            new_coordinate.x = 1;
        } else if new_coordinate.x == 0 {
            new_coordinate.x = max_x - 1;
        }

        if new_coordinate.y == max_y {
            new_coordinate.y = 1;
        } else if new_coordinate.y == 0 {
            new_coordinate.y = max_y - 1;
        }

        new_coordinate
    }
}
