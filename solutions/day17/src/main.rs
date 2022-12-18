use std::{collections::HashSet, fmt::Display};

const START_X: usize = 3;
const WIDTH: usize = 9;
const AIR_ROW: [bool; WIDTH] = [true, false, false, false, false, false, false, false, true];

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn translate_new(&self, dx: isize, dy: isize) -> Self {
        Self::new(
            (self.x as isize + dx) as usize,
            (self.y as isize + dy) as usize,
        )
    }
}

struct Rock {
    coordinates: HashSet<Coordinate>,
}

impl Rock {
    fn max_y(&self) -> usize {
        self.coordinates.iter().map(|r| r.y).max().unwrap()
    }
    fn new(rocks_dropped: usize, bottom: usize) -> Self {
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

struct Chamber {
    grid: Vec<[bool; WIDTH]>,
    top: usize,
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
    fn new() -> Self {
        let grid = vec![[true; WIDTH]];

        Self {
            grid,
            top: 0,
            rock: Rock::new(0, 0),
        }
    }

    fn start_dropping_rock(&mut self, r: usize) {
        self.rock = Rock::new(r, self.top + 4);
        self.top = self.rock.max_y();
        self.grid.resize(self.top + 1, AIR_ROW);
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
        self.top = (self.top as isize + dy) as usize;
        true
    }

    fn move_rock_right(&mut self) {
        self.move_rock(1, 0);
    }

    fn move_rock_left(&mut self) {
        self.move_rock(-1, 0);
    }

    fn move_rock_down(&mut self) -> bool {
        self.move_rock(0, -1)
    }
}

fn problem1(input: &str) -> usize {
    let mut chamber = Chamber::new();
    let mut dir_iter = input.trim_end().chars().cycle();
    for r in 0..2022 {
        dbg!(r);
        chamber.start_dropping_rock(r);
        loop {
            match dir_iter.next().unwrap() {
                '>' => chamber.move_rock_right(),
                '<' => chamber.move_rock_left(),
                _ => unreachable!("Invalid input!"),
            }
            if !chamber.move_rock_down() {
                break;
            }
        }
    }
    println!("{}", &chamber);
    chamber.top
}

fn problem2(input: &str) -> usize {
    unimplemented!()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 3068);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 0);
}
