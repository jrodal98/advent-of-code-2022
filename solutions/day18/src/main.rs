use std::collections::HashSet;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

#[derive(Debug, Hash, PartialEq, Eq, sscanf::FromScanf)]
#[sscanf(format = "{x},{y},{z}")]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

impl Coordinate {
    fn adjacent(&self, other: &Self) -> bool {
        self != other
            && match (
                self.x.abs_diff(other.x),
                self.y.abs_diff(other.y),
                self.z.abs_diff(other.z),
            ) {
                (1, 0, 0) => true,
                (0, 1, 0) => true,
                (0, 0, 1) => true,
                _ => false,
            }
    }

    fn translate_new(&self, dx: isize, dy: isize, dz: isize) -> Self {
        Coordinate {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }

    fn adjacent_coordinates(&self) -> Vec<Self> {
        vec![
            self.translate_new(1, 0, 0),
            self.translate_new(-1, 0, 0),
            self.translate_new(0, 1, 0),
            self.translate_new(0, -1, 0),
            self.translate_new(0, 0, 1),
            self.translate_new(0, 0, -1),
        ]
    }
}

struct Grid {
    coordinates: HashSet<Coordinate>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let coordinates: HashSet<Coordinate> = input
            .lines()
            .map(|line| sscanf::sscanf!(line, "{Coordinate}").unwrap())
            .collect();
        Self { coordinates }
    }

    fn sa_including_air_pockets(&self) -> usize {
        self.coordinates.len() * 6
            - self
                .coordinates
                .iter()
                .flat_map(|c| c.adjacent_coordinates())
                .filter(|c| self.coordinates.contains(c))
                .count()
    }

    fn get_air_pockets(&self) -> HashSet<Coordinate> {
        let min_x = self.coordinates.iter().map(|c| c.x).min().unwrap_or(0);
        let max_x = self.coordinates.iter().map(|c| c.x).max().unwrap_or(0);
        let min_y = self.coordinates.iter().map(|c| c.y).min().unwrap_or(0);
        let max_y = self.coordinates.iter().map(|c| c.y).max().unwrap_or(0);
        let min_z = self.coordinates.iter().map(|c| c.z).min().unwrap_or(0);
        let max_z = self.coordinates.iter().map(|c| c.z).max().unwrap_or(0);

        let mut pockets: HashSet<Coordinate> = (min_x - 1..=max_x + 1)
            .flat_map(|x| (min_y - 1..=max_y + 1).map(move |y| (x, y)))
            .flat_map(|(x, y)| (min_z - 1..=max_z + 1).map(move |z| Coordinate { x, y, z }))
            .filter(|coord| !self.coordinates.contains(coord))
            .collect();

        let mut cubes_to_explore = vec![Coordinate {
            x: min_x - 1,
            y: min_y - 1,
            z: min_z - 1,
        }];

        // Simulate "being the water"
        while let Some(cube) = cubes_to_explore.pop() {
            // if the water is able to reach the cube, then it isn't an air pocket
            if pockets.remove(&cube) {
                // continue exploring along the space
                cubes_to_explore.extend(cube.adjacent_coordinates());
            }
        }

        pockets
    }

    fn sa_excluding_air_pockets(&self) -> usize {
        let including_pockets = self.sa_including_air_pockets();
        let air_pockets = self.get_air_pockets();

        let air_pocket_penalty = air_pockets.len() * 6
            - air_pockets
                .iter()
                .flat_map(|c| c.adjacent_coordinates())
                .filter(|c| air_pockets.contains(c))
                .count();

        including_pockets - air_pocket_penalty
    }
}

fn problem1(input: &str) -> usize {
    Grid::new(input).sa_including_air_pockets()
}

fn problem2(input: &str) -> usize {
    Grid::new(input).sa_excluding_air_pockets()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 64);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 58);
}
