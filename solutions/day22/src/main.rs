use std::{collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

#[derive(Debug)]
enum Rotation {
    L,
    R,
    N,
}

#[derive(Debug)]
enum Movement {
    U(u8),
    D(u8),
    L(u8),
    R(u8),
}

impl From<Movement> for u32 {
    fn from(value: Movement) -> Self {
        match value {
            Movement::U(_) => 3,
            Movement::D(_) => 1,
            Movement::L(_) => 2,
            Movement::R(_) => 0,
        }
    }
}

impl Movement {
    fn new_with_rotation(last_movement: &Movement, rotation: Rotation, steps: u8) -> Self {
        match (last_movement, rotation) {
            (Movement::U(_), Rotation::L) => Movement::L(steps),
            (Movement::U(_), Rotation::R) => Movement::R(steps),
            (Movement::U(_), Rotation::N) => Movement::U(steps),
            (Movement::D(_), Rotation::L) => Movement::R(steps),
            (Movement::D(_), Rotation::R) => Movement::L(steps),
            (Movement::D(_), Rotation::N) => Movement::D(steps),
            (Movement::L(_), Rotation::L) => Movement::D(steps),
            (Movement::L(_), Rotation::R) => Movement::U(steps),
            (Movement::L(_), Rotation::N) => Movement::L(steps),
            (Movement::R(_), Rotation::L) => Movement::U(steps),
            (Movement::R(_), Rotation::R) => Movement::D(steps),
            (Movement::R(_), Rotation::N) => Movement::R(steps),
        }
    }

    pub fn translate_path_description(description: &str) -> Vec<Self> {
        let mut chars = description.chars().peekable();
        let mut movements = Vec::new();
        let mut next_rotation = Rotation::N;
        while chars.peek().is_some() {
            let mut steps = 0;
            let mut rotation: Option<char> = None;
            while rotation.is_none() {
                rotation = match chars.next() {
                    Some('L') => Some('L'),
                    Some('R') => Some('R'),
                    Some(x) => {
                        steps = steps * 10 + x.to_digit(10).unwrap() as u8;
                        None
                    }
                    None => break,
                }
            }

            let last_movement = movements.last();
            let new_movement = Self::new_with_rotation(
                // start facing right
                last_movement.unwrap_or(&Movement::R(0)),
                next_rotation,
                steps,
            );

            movements.push(new_movement);
            next_rotation = Rotation::new(rotation);
        }
        movements
    }
}

impl Rotation {
    fn new(rotation: Option<char>) -> Self {
        match rotation {
            Some('L') => Self::L,
            Some('R') => Self::R,
            _ => Self::N,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coordinate {
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct Board {
    explorer: Coordinate,
    empty_tiles: HashSet<Coordinate>,
    wall_tiles: HashSet<Coordinate>,
}

impl Board {
    pub fn explore(&mut self, movement: Movement) -> u32 {
        let (steps, dx, dy) = match &movement {
            &Movement::U(steps) => (steps, 0, -1),
            &Movement::D(steps) => (steps, 0, 1),
            &Movement::L(steps) => (steps, -1, 0),
            &Movement::R(steps) => (steps, 1, 0),
        };

        for _ in 0..steps {
            let new_position = self
                .explorer
                .try_new_from_translation(dx, dy)
                .unwrap_or_else(|| self.teleport(&movement));

            if self.wall_tiles.contains(&new_position) {
                break;
            } else if self.empty_tiles.contains(&new_position) {
                self.explorer = new_position;
            } else {
                let new_position = self.teleport(&movement);
                if self.wall_tiles.contains(&new_position) {
                    break;
                } else {
                    self.explorer = new_position;
                }
            }
        }

        let movement_score = u32::from(movement);
        let column_score = 4 * (self.explorer.x as u32 + 1);
        let row_score = 1000 * (self.explorer.y as u32 + 1);

        movement_score + column_score + row_score
    }

    fn teleport(&self, movement: &Movement) -> Coordinate {
        let all_points = self.wall_tiles.union(&self.empty_tiles);
        let new_coordinate = match &movement {
            Movement::U(_) => {
                let y = all_points
                    .into_iter()
                    .filter(|&c| c.x == self.explorer.x)
                    .map(|&c| c.y)
                    .max()
                    .unwrap();
                Coordinate::new(self.explorer.x, y)
            }
            Movement::D(_) => {
                let y = all_points
                    .into_iter()
                    .filter(|&c| c.x == self.explorer.x)
                    .map(|&c| c.y)
                    .min()
                    .unwrap();
                Coordinate::new(self.explorer.x, y)
            }
            Movement::L(_) => {
                let x = all_points
                    .into_iter()
                    .filter(|&c| c.y == self.explorer.y)
                    .map(|&c| c.x)
                    .max()
                    .unwrap();
                Coordinate::new(x, self.explorer.y)
            }
            Movement::R(_) => {
                let x = all_points
                    .into_iter()
                    .filter(|&c| c.y == self.explorer.y)
                    .map(|&c| c.x)
                    .min()
                    .unwrap();
                Coordinate::new(x, self.explorer.y)
            }
        };

        new_coordinate
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut empty_tiles = HashSet::new();
        let mut wall_tiles = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => &mut empty_tiles,
                    '#' => &mut wall_tiles,
                    _ => continue,
                }
                .insert(Coordinate::new(x as u8, y as u8));
            }
        }

        let explorer_x = empty_tiles
            .iter()
            .filter_map(|c| if c.y == 0 { Some(c.x) } else { None })
            .min()
            .unwrap();
        let explorer = Coordinate::new(explorer_x, 0);

        Ok(Self {
            explorer,
            empty_tiles,
            wall_tiles,
        })
    }
}

impl Coordinate {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    fn try_new_from_translation(&self, dx: i16, dy: i16) -> Option<Self> {
        let x = self.x as i16 + dx;
        let y = self.y as i16 + dy;

        if x < 0 || y < 0 || x > u8::MAX as i16 || y > u8::MAX as i16 {
            None
        } else {
            Some(Self::new(x as u8, y as u8))
        }
    }
}

fn problem1(input: &str) -> u32 {
    let (mut board, movements) = input
        .split_once("\n\n")
        .map(|(board_str, movements_str)| {
            (
                board_str.parse::<Board>().unwrap(),
                Movement::translate_path_description(movements_str.trim()),
            )
        })
        .unwrap();

    let mut score = 0;
    for movement in movements {
        score = board.explore(movement);
    }

    // 139188 is too high
    // answer is 76322

    score
}

fn problem2(input: &str) -> u32 {
    unimplemented!()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 6032);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 0);
}
