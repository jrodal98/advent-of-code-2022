use std::collections::HashSet;

use crate::{coordinate::Coordinate, direction::Direction};

#[derive(Clone, Debug)]
pub struct Blizzard {
    winds: HashSet<Coordinate>,
    direction: Direction,
}

impl Blizzard {
    pub fn new(winds: HashSet<Coordinate>, direction: Direction) -> Self {
        Self { winds, direction }
    }

    pub fn blow_winds(&mut self, max_x: u8, max_y: u8) {
        self.winds = self
            .winds
            .iter()
            .map(|c| c.blow(max_x, max_y, self.direction))
            .collect();
    }

    pub fn contains(&self, coordinate: &Coordinate) -> bool {
        self.winds.contains(coordinate)
    }
}
