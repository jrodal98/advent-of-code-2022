use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{blizzard::Blizzard, coordinate::Coordinate, direction::Direction};

const GRID_ENTRANCE: Coordinate = Coordinate { x: 1, y: 0 };

#[derive(Debug, Clone)]
pub struct Grid {
    expedition: Coordinate,
    start: Coordinate,
    exit: Coordinate,
    blizzards: [Blizzard; 4],
    max_x: u8,
    max_y: u8,
    prefer_down_right: bool,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left_blizzards: HashSet<Coordinate> = HashSet::new();
        let mut right_blizzards: HashSet<Coordinate> = HashSet::new();
        let mut up_blizzards: HashSet<Coordinate> = HashSet::new();
        let mut down_blizzards: HashSet<Coordinate> = HashSet::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coordinate = Coordinate::from_usizes(x, y);
                match Direction::from(c) {
                    Direction::Up => up_blizzards.insert(coordinate),
                    Direction::Down => down_blizzards.insert(coordinate),
                    Direction::Left => left_blizzards.insert(coordinate),
                    Direction::Right => right_blizzards.insert(coordinate),
                    Direction::Stay => false,
                };
            }
        }

        let max_x = s.lines().next().unwrap_or_default().chars().count() as u8 - 1;
        let max_y = s.lines().count() as u8 - 1;

        let exit = Coordinate::new(max_x - 1, max_y);
        let expedition = GRID_ENTRANCE;
        let start = GRID_ENTRANCE;
        let prefer_down_right = true;

        let blizzards = [
            Blizzard::new(left_blizzards, Direction::Left),
            Blizzard::new(right_blizzards, Direction::Right),
            Blizzard::new(down_blizzards, Direction::Down),
            Blizzard::new(up_blizzards, Direction::Up),
        ];

        Ok(Self {
            expedition,
            start,
            exit,
            max_x,
            max_y,
            blizzards,
            prefer_down_right,
        })
    }
}

impl Grid {
    fn blow_winds(&mut self) {
        self.blizzards
            .iter_mut()
            .for_each(|b| b.blow_winds(self.max_x, self.max_y))
    }

    pub fn find_fastest_exit(&self) -> usize {
        if self.expedition == self.exit {
            0
        } else {
            let mut best_so_far = usize::MAX;
            self.clone()
                .leave_grid(0, &mut best_so_far, &mut HashMap::new())
        }
    }

    pub fn round_trip_for_snacks(&self) -> usize {
        let mut grid_clone = self.clone();
        dbg!("Going from start -> exit");
        // dbg!(&grid_clone);
        let fastest_exit = grid_clone.find_fastest_exit();
        for _ in 0..fastest_exit {
            grid_clone.blow_winds();
        }
        grid_clone.start = self.exit;
        grid_clone.expedition = self.exit;
        grid_clone.exit = self.start;
        grid_clone.prefer_down_right = false;

        dbg!("Going from exit -> start");
        // dbg!(&grid_clone);

        let fastest_entrance = grid_clone.find_fastest_exit();

        for _ in 0..fastest_entrance {
            grid_clone.blow_winds();
        }

        grid_clone.start = self.start;
        grid_clone.expedition = self.start;
        grid_clone.exit = self.exit;
        grid_clone.prefer_down_right = true;

        dbg!("Going from start -> exit");
        // dbg!(&grid_clone);

        // this is very stupid (I already have a good idea of what the fastest exit will be)
        // but I'm tired
        fastest_exit + fastest_entrance + grid_clone.find_fastest_exit()
    }

    fn is_valid_position(&self, coordinate: &Coordinate) -> bool {
        *coordinate == self.start
            || (coordinate.x > 0
                && coordinate.y > 0
                && coordinate.x < self.max_x
                && coordinate.y < self.max_y
                && !self.blizzards.iter().any(|b| b.contains(coordinate)))
    }

    fn leave_grid(
        mut self,
        time_passed: usize,
        best_so_far: &mut usize,
        cache: &mut HashMap<(Coordinate, usize), usize>,
    ) -> usize {
        let key = (self.expedition.clone(), time_passed);
        if let Some(cached_result) = cache.get(&key) {
            return *cached_result;
        }
        if time_passed + (self.expedition.distance(&self.exit) as usize) >= *best_so_far {
            return usize::MAX;
        }

        let mut best_from_here = usize::MAX;

        // we are one step away (and should go down this turn)
        if self.expedition.distance(&self.exit) == 1 {
            best_from_here = time_passed + 1;
        } else {
            self.blow_winds();
            if self.expedition == self.start {
                let preferred_direction = if self.prefer_down_right {
                    Direction::Down
                } else {
                    Direction::Up
                };
                let new_direction = if self
                    .is_valid_position(&self.expedition.move_in_direction(&preferred_direction))
                {
                    preferred_direction
                } else {
                    Direction::Stay
                };
                let mut new_grid = self.clone();
                new_grid.expedition = self.expedition.move_in_direction(&new_direction);
                let new_direction_best = new_grid.leave_grid(time_passed + 1, best_so_far, cache);
                best_from_here = best_from_here.min(new_direction_best);
            } else {
                for movement in Direction::iterator(self.prefer_down_right)
                    .map(|d| self.expedition.move_in_direction(&d))
                    .filter(|c| self.is_valid_position(&c))
                {
                    let mut new_grid = self.clone();
                    new_grid.expedition = movement;
                    let new_direction_best =
                        new_grid.leave_grid(time_passed + 1, best_so_far, cache);
                    best_from_here = best_from_here.min(new_direction_best);
                }
            }
        }

        if best_from_here < *best_so_far {
            *best_so_far = best_from_here;
            println!("{}", &best_so_far);
        }
        cache.insert(key, best_from_here);
        best_from_here
    }
}
