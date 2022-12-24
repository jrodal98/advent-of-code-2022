use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

const GRID_ENTRANCE: Coordinate = Coordinate { x: 1, y: 0 };

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stay,
}

impl Direction {
    pub fn iterator() -> impl Iterator<Item = Self> {
        [Self::Down, Self::Right, Self::Stay, Self::Up, Self::Left]
            .iter()
            .copied()
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => Self::Stay,
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
    fn from_usizes(x: usize, y: usize) -> Self {
        Self {
            x: x as u8,
            y: y as u8,
        }
    }
    fn move_in_direction(self, direction: &Direction) -> Self {
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

    fn distance(&self, other: &Self) -> u8 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn blow(self, max_x: u8, max_y: u8, wind: Direction) -> Self {
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

#[derive(Debug, Clone)]
struct Grid {
    expedition: Coordinate,
    exit: Coordinate,
    blizzards: [Blizzard; 4],
    max_x: u8,
    max_y: u8,
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

        let blizzards = [
            Blizzard::new(left_blizzards, Direction::Left),
            Blizzard::new(right_blizzards, Direction::Right),
            Blizzard::new(down_blizzards, Direction::Down),
            Blizzard::new(up_blizzards, Direction::Up),
        ];

        Ok(Self {
            expedition,
            exit,
            max_x,
            max_y,
            blizzards,
        })
    }
}

#[derive(Clone, Debug)]
struct Blizzard {
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

    fn is_valid_position(&self, coordinate: &Coordinate) -> bool {
        *coordinate == GRID_ENTRANCE
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
            if self.expedition == GRID_ENTRANCE {
                let new_direction = if self
                    .is_valid_position(&self.expedition.move_in_direction(&Direction::Down))
                {
                    Direction::Down
                } else {
                    Direction::Stay
                };
                let mut new_grid = self.clone();
                new_grid.expedition = self.expedition.move_in_direction(&new_direction);
                let new_direction_best = new_grid.leave_grid(time_passed + 1, best_so_far, cache);
                best_from_here = best_from_here.min(new_direction_best);
            } else {
                for movement in Direction::iterator()
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

fn problem1(input: &str) -> u32 {
    input.parse::<Grid>().unwrap().find_fastest_exit() as u32
}

fn problem2(input: &str) -> u32 {
    unimplemented!()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 18);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 0);
}
