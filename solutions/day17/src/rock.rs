use std::collections::HashSet;

use crate::coordinate::Coordinate;

const START_X: usize = 3;

pub struct Rock {
    pub coordinates: HashSet<Coordinate>,
}

impl Rock {
    pub fn max_y(&self) -> usize {
        self.coordinates.iter().map(|r| r.y).max().unwrap_or(0)
    }
    pub fn new(rocks_dropped: usize, bottom: usize) -> Self {
        let coordinates = match rocks_dropped % 5 {
            0 => (START_X..=START_X + 3)
                .map(|x| Coordinate::new(x, bottom))
                .collect(),
            1 => HashSet::from_iter(
                vec![
                    Coordinate::new(START_X + 1, bottom),
                    Coordinate::new(START_X, bottom + 1),
                    Coordinate::new(START_X + 1, bottom + 1),
                    Coordinate::new(START_X + 2, bottom + 1),
                    Coordinate::new(START_X + 1, bottom + 2),
                ]
                .into_iter(),
            ),
            2 => HashSet::from_iter(
                vec![
                    Coordinate::new(START_X, bottom),
                    Coordinate::new(START_X + 1, bottom),
                    Coordinate::new(START_X + 2, bottom),
                    Coordinate::new(START_X + 2, bottom + 1),
                    Coordinate::new(START_X + 2, bottom + 2),
                ]
                .into_iter(),
            ),
            3 => (0..=3)
                .map(|y_offset| Coordinate::new(START_X, bottom + y_offset))
                .collect(),
            4 => HashSet::from_iter(
                vec![
                    Coordinate::new(START_X, bottom),
                    Coordinate::new(START_X + 1, bottom),
                    Coordinate::new(START_X, bottom + 1),
                    Coordinate::new(START_X + 1, bottom + 1),
                ]
                .into_iter(),
            ),
            _ => unreachable!(),
        };

        Self { coordinates }
    }
}
