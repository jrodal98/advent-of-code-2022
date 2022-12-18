use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl FromStr for Coordinate {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, yz) = s.split_once(",").unwrap();
        let (y, z) = yz.split_once(",").unwrap();
        Ok(Coordinate {
            x: x.parse()?,
            y: y.parse()?,
            z: z.parse()?,
        })
    }
}

impl Coordinate {
    pub fn translate_new(&self, dx: i8, dy: i8, dz: i8) -> Self {
        Coordinate {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }

    pub fn adjacent_coordinates(&self) -> [Coordinate; 6] {
        [
            self.translate_new(1, 0, 0),
            self.translate_new(-1, 0, 0),
            self.translate_new(0, 1, 0),
            self.translate_new(0, -1, 0),
            self.translate_new(0, 0, 1),
            self.translate_new(0, 0, -1),
        ]
    }
}
