use std::{collections::HashSet, fmt::Display};

use crate::{coordinate::Coordinate, rock::Rock};

const WIDTH: usize = 9;
const AIR_ROW: [bool; WIDTH] = [true, false, false, false, false, false, false, false, true];

pub struct Chamber {
    grid: Vec<[bool; WIDTH]>,
    rock: Rock,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid_string = String::new();
        for (i, r) in self.grid.iter().rev().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                let bottom_row = i == self.grid.len() - 1;
                let wall = j == 0 || j == WIDTH - 1;
                let ch = if bottom_row && wall {
                    "+"
                } else if bottom_row {
                    "-"
                } else if wall {
                    "|"
                } else if c {
                    "#"
                } else {
                    "."
                };
                grid_string.push_str(ch);
            }
            grid_string.push_str("\n");
        }
        write!(f, "{}", grid_string)
    }
}

impl Chamber {
    pub fn new() -> Self {
        let grid = vec![[true; WIDTH]];

        Self {
            grid,
            rock: Rock {
                coordinates: HashSet::new(),
            },
        }
    }

    pub fn top(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .rev()
            .find(|(_, r)| (1..WIDTH - 1).any(|i| r[i]))
            .unwrap()
            .0
    }

    pub fn start_dropping_rock(&mut self, r: usize) {
        self.rock = Rock::new(r, self.top() + 4);
        self.grid.resize(self.rock.max_y() + 1, AIR_ROW);
        for c in self.rock.coordinates.iter() {
            self.grid[c.y][c.x] = true;
        }
    }

    fn move_rock(&mut self, dx: isize, dy: isize) -> bool {
        // dbg!((dx, dy), &self.rock.coordinates);
        let new_coords: HashSet<Coordinate> = self
            .rock
            .coordinates
            .iter()
            .map(|c| c.translate_new(dx, dy))
            .collect();

        // if there is a collision at a coordinate
        // that did not belong to the rock, don't move the rock
        if new_coords
            .difference(&self.rock.coordinates)
            .into_iter()
            .any(|c| self.grid[c.y][c.x])
        {
            return false;
        }

        // set old coordinates as unoccupied
        self.rock
            .coordinates
            .iter()
            .for_each(|c| self.grid[c.y][c.x] = false);

        // replace coordinates
        self.rock.coordinates = new_coords;

        // set new coordinates as occupied
        self.rock
            .coordinates
            .iter()
            .for_each(|c| self.grid[c.y][c.x] = true);
        true
    }

    pub fn move_rock_right(&mut self) {
        self.move_rock(1, 0);
    }

    pub fn move_rock_left(&mut self) {
        self.move_rock(-1, 0);
    }

    pub fn move_rock_down(&mut self) -> bool {
        self.move_rock(0, -1)
    }
}
