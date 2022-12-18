fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

#[derive(Debug, PartialEq, Eq, sscanf::FromScanf)]
#[sscanf(format = "{x},{y},{z}")]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
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
}

struct Grid {
    coordinates: Vec<Coordinate>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let coordinates: Vec<Coordinate> = input
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
                .map(|c| self.coordinates.iter().filter(|o| c.adjacent(o)).count())
                .sum::<usize>()
    }

    fn num_air_pockets(&self) -> usize {
        1
    }

    fn sa_excluding_air_pockets(&self) -> usize {
        self.sa_including_air_pockets() - (self.num_air_pockets() * 6)
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
