use std::{collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!(
        "Problem 2: {}",
        problem2(input, TeleportationStrategy::Part2MyInput)
    );
}

#[derive(Debug)]
enum Rotation {
    L,
    R,
    N,
}

enum TeleportationStrategy {
    Part1,
    Part2Sample,
    Part2MyInput,
}

#[derive(Clone, Copy, Debug)]
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
    fn unpack_translation(&self) -> (u8, i16, i16) {
        match &self {
            &Movement::U(steps) => (*steps, 0, -1),
            &Movement::D(steps) => (*steps, 0, 1),
            &Movement::L(steps) => (*steps, -1, 0),
            &Movement::R(steps) => (*steps, 1, 0),
        }
    }
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
    pub fn explore(&mut self, mut movement: Movement, strategy: &TeleportationStrategy) -> u32 {
        let (steps, mut dx, mut dy) = movement.unpack_translation();

        for _ in 0..steps {
            dbg!(("Before", &movement, &dx, &dy, &self.explorer));
            let new_position = self
                .explorer
                .try_new_from_translation(dx, dy)
                .unwrap_or_else(|| self.teleport(&mut movement, &mut dx, &mut dy, strategy));

            if self.wall_tiles.contains(&new_position) {
                dbg!(("After", &movement, &dx, &dy, &self.explorer));
                break;
            } else if self.empty_tiles.contains(&new_position) {
                self.explorer = new_position;
                dbg!(("After", &movement, &dx, &dy, &self.explorer));
            } else {
                let new_position = self.teleport(&mut movement, &mut dx, &mut dy, strategy);
                if self.wall_tiles.contains(&new_position) {
                    dbg!(("After", &movement, &dx, &dy, &self.explorer));
                    break;
                } else {
                    self.explorer = new_position;
                    dbg!(("After", &movement, &dx, &dy, &self.explorer));
                }
            }
        }

        let movement_score = u32::from(movement);
        let column_score = 4 * (self.explorer.x as u32 + 1);
        let row_score = 1000 * (self.explorer.y as u32 + 1);

        movement_score + column_score + row_score
    }

    fn teleport(
        &self,
        movement: &mut Movement,
        dx: &mut i16,
        dy: &mut i16,
        strategy: &TeleportationStrategy,
    ) -> Coordinate {
        let (coordinate, new_movement) = match strategy {
            TeleportationStrategy::Part1 => self.teleport_part1(movement),
            TeleportationStrategy::Part2Sample => self.teleport_part2_sample(movement),
            TeleportationStrategy::Part2MyInput => self.teleport_part2_my_input(movement),
        };

        *movement = new_movement;
        (_, *dx, *dy) = movement.unpack_translation();
        coordinate
    }

    fn teleport_part1(&self, movement: &Movement) -> (Coordinate, Movement) {
        let all_points = self.wall_tiles.union(&self.empty_tiles);
        match &movement {
            Movement::U(_) => {
                let y = all_points
                    .into_iter()
                    .filter(|&c| c.x == self.explorer.x)
                    .map(|&c| c.y)
                    .max()
                    .unwrap();
                (Coordinate::new(self.explorer.x, y), movement.clone())
            }
            Movement::D(_) => {
                let y = all_points
                    .into_iter()
                    .filter(|&c| c.x == self.explorer.x)
                    .map(|&c| c.y)
                    .min()
                    .unwrap();
                (Coordinate::new(self.explorer.x, y), movement.clone())
            }
            Movement::L(_) => {
                let x = all_points
                    .into_iter()
                    .filter(|&c| c.y == self.explorer.y)
                    .map(|&c| c.x)
                    .max()
                    .unwrap();
                (Coordinate::new(x, self.explorer.y), movement.clone())
            }
            Movement::R(_) => {
                let x = all_points
                    .into_iter()
                    .filter(|&c| c.y == self.explorer.y)
                    .map(|&c| c.x)
                    .min()
                    .unwrap();
                (Coordinate::new(x, self.explorer.y), movement.clone())
            }
        }
    }

    fn teleport_part2_sample(&self, movement: &mut Movement) -> (Coordinate, Movement) {
        let (col, row) = (self.explorer.x, self.explorer.y);
        match (row, col, movement) {
            // Face 1
            (0..=3, 8..=11, Movement::U(m)) => (Coordinate::new(11 - row, 4), Movement::D(*m)),
            (0..=3, 8..=11, Movement::L(m)) => (Coordinate::new(row + 4, 4), Movement::D(*m)),
            (0..=3, 8..=11, Movement::R(m)) => (Coordinate::new(15, 11 - row), Movement::L(*m)),
            // Face 2
            (4..=7, 0..=3, Movement::D(m)) => (Coordinate::new(11 - col, 11), Movement::U(*m)),
            (4..=7, 0..=3, Movement::L(m)) => (Coordinate::new(19 - row, 11), Movement::U(*m)),
            (4..=7, 0..=3, Movement::U(m)) => (Coordinate::new(12 - col, 0), Movement::D(*m)),
            // Face 3
            (4..=7, 4..=7, Movement::D(m)) => (Coordinate::new(8, 15 - col), Movement::R(*m)),
            (4..=7, 4..=7, Movement::U(m)) => (Coordinate::new(8, col - 2), Movement::R(*m)),
            // Face 4
            (4..=7, 8..=11, Movement::R(m)) => (Coordinate::new(19 - row, 8), Movement::D(*m)),
            // Face 5
            (8..=11, 8..=11, Movement::D(m)) => (Coordinate::new(11 - col, 7), Movement::U(*m)),
            (8..=11, 8..=11, Movement::L(m)) => (Coordinate::new(15 - row, 7), Movement::U(*m)),
            // Face 6
            (8..=11, 12..=15, Movement::R(m)) => (Coordinate::new(11, 11 - row), Movement::L(*m)),
            (8..=11, 12..=15, Movement::D(m)) => (Coordinate::new(0, 19 - col), Movement::R(*m)),
            (8..=11, 12..=15, Movement::U(m)) => (Coordinate::new(11, 19 - col), Movement::L(*m)),
            _ => unreachable!(),
        }
    }

    fn teleport_part2_my_input(&self, movement: &mut Movement) -> (Coordinate, Movement) {
        todo!()
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

fn solve(input: &str, strategy: TeleportationStrategy) -> u32 {
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
        score = board.explore(movement, &strategy);
    }

    score
}

fn problem1(input: &str) -> u32 {
    solve(input, TeleportationStrategy::Part1)
}

fn problem2(input: &str, strategy: TeleportationStrategy) -> u32 {
    solve(input, strategy)
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
    let res = problem2(input, TeleportationStrategy::Part2Sample);
    assert_eq!(res, 5031);
}
