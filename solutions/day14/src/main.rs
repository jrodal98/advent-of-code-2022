use std::{fmt::Display, str::FromStr};

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Clone)]
enum Block {
    Air,
    Rock,
    Sand,
    Generator,
}

impl Block {
    fn to_string(&self) -> &str {
        match &self {
            Block::Air => ".",
            Block::Rock => "#",
            Block::Sand => "o",
            Block::Generator => "+",
        }
    }
}

struct Grid {
    num_rows: usize,
    num_cols: usize,
    sand_generator: (usize, usize),
    rows: Vec<Vec<Block>>,
}

impl Grid {
    fn new(
        rocks: Vec<Coordinate>,
        sand_generator: Coordinate,
        num_rows: usize,
        num_cols: usize,
    ) -> Self {
        let mut rows = vec![vec![Block::Air; num_cols]; num_rows];
        for rock_coordinate in rocks {
            rows[rock_coordinate.y][rock_coordinate.x] = Block::Rock;
        }
        rows[sand_generator.y][sand_generator.x] = Block::Generator;
        Self {
            num_rows,
            num_cols,
            rows,
            sand_generator: (sand_generator.y, sand_generator.x),
        }
    }

    fn drop_sand_block(&mut self) -> bool {
        let (mut i, mut j) = self.sand_generator;

        while i < self.num_rows - 1 && j > 0 && j < self.num_cols - 1 {
            i = i + 1;
            let down_block = &self.rows[i][j];
            let left_block = &self.rows[i][j - 1];
            let right_block = &self.rows[i][j + 1];

            match (left_block, down_block, right_block) {
                (_, Block::Air, _) => continue,
                (Block::Air, _, _) => {
                    j = j - 1;
                    continue;
                }
                (_, _, Block::Air) => {
                    j = j + 1;
                    continue;
                }
                (_, _, _) => {
                    self.rows[i - 1][j] = Block::Sand;
                    return (i - 1, j) != self.sand_generator;
                }
            }
        }
        false
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid_string = String::new();
        for r in self.rows.iter() {
            for c in r.iter() {
                grid_string.push_str(c.to_string());
            }
            grid_string.push_str("\n");
        }
        write!(f, "{}", grid_string)
    }
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(",")
            .and_then(|(s1, s2)| Some((s1.parse().unwrap(), s2.parse().unwrap())))
            .unwrap();
        Ok(Self { x, y })
    }
}

impl Coordinate {
    fn gen_coordinates(&self, other: &Self, dx: usize) -> Vec<Coordinate> {
        // vertical line
        if self.x == other.x {
            let min = self.y.min(other.y);
            let max = self.y.max(other.y);
            (min..=max)
                .into_iter()
                .map(|y| Coordinate { x: self.x - dx, y })
                .collect()
        } else {
            let min = self.x.min(other.x);
            let max = self.x.max(other.x);
            (min..=max)
                .into_iter()
                .map(|x| Coordinate {
                    x: x - dx,
                    y: self.y,
                })
                .collect()
        }
    }
}

fn drop_sand(mut grid: Grid) -> u32 {
    for ans in 0.. {
        if !grid.drop_sand_block() {
            return ans;
        }
    }
    unreachable!()
}

fn create_grid(input: &str, add_infinite_bottom: bool) -> Grid {
    // get coordinates, normalize them, and then use window to create lines
    let coordinates: Vec<Vec<Coordinate>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| coord.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let (min_x, max_x, max_y) = if add_infinite_bottom {
        let max_y = coordinates.iter().flatten().map(|c| c.y).max().unwrap() + 2;
        let min_x = 0;
        // create "infinite" horizontal line
        let max_x = coordinates.iter().flatten().map(|c| c.x).max().unwrap() + max_y;
        (min_x, max_x, max_y)
    } else {
        let min_x = coordinates.iter().flatten().map(|c| c.x).min().unwrap();
        let max_x = coordinates.iter().flatten().map(|c| c.x).max().unwrap();
        let max_y = coordinates.iter().flatten().map(|c| c.y).max().unwrap();
        (min_x, max_x, max_y)
    };

    let sand_generator_coordinate = Coordinate {
        x: 500 - min_x,
        y: 0,
    };
    let mut rock_coordinates: Vec<Coordinate> = coordinates
        .into_iter()
        .flat_map(|v| {
            v.windows(2)
                .flat_map(|w| w[0].gen_coordinates(&w[1], min_x))
                .collect::<Vec<Coordinate>>()
        })
        .collect();

    if add_infinite_bottom {
        rock_coordinates.append(
            &mut Coordinate { x: min_x, y: max_y }
                .gen_coordinates(&Coordinate { x: max_x, y: max_y }, min_x),
        );
    }

    Grid::new(
        rock_coordinates,
        sand_generator_coordinate,
        max_y + 1,
        max_x - min_x + 1,
    )
}

fn problem1(input: &str) -> u32 {
    let grid = create_grid(input, false);
    drop_sand(grid)
}

fn problem2(input: &str) -> u32 {
    let grid = create_grid(input, true);
    drop_sand(grid) + 1
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 24);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 93);
}
