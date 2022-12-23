use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const NUM_ROUNDS: usize = 10;

#[derive(Copy, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
    NE,
    NW,
    SE,
    SW,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn translate_new(&self, direction: Direction) -> Self {
        match direction {
            Direction::N => Self::new(self.x, self.y - 1),
            Direction::E => Self::new(self.x + 1, self.y),
            Direction::S => Self::new(self.x, self.y + 1),
            Direction::W => Self::new(self.x - 1, self.y),
            Direction::NE => Self::new(self.x + 1, self.y - 1),
            Direction::NW => Self::new(self.x - 1, self.y - 1),
            Direction::SE => Self::new(self.x + 1, self.y + 1),
            Direction::SW => Self::new(self.x - 1, self.y + 1),
        }
    }
}

struct Grid {
    elves: HashSet<Coordinate>,
}

impl Grid {
    fn propose_elf(&self, elf: &Coordinate, round: usize) -> Option<Coordinate> {
        let n_translation = elf.translate_new(Direction::N);
        let e_translation = elf.translate_new(Direction::E);
        let s_translation = elf.translate_new(Direction::S);
        let w_translation = elf.translate_new(Direction::W);
        let ne_translation = elf.translate_new(Direction::NE);
        let nw_translation = elf.translate_new(Direction::NW);
        let se_translation = elf.translate_new(Direction::SE);
        let sw_translation = elf.translate_new(Direction::SW);

        let mut proposals: [Option<Coordinate>; 4] = [None; 4];
        let mut num_proposals = 0;

        if !self.elves.contains(&n_translation)
            && !self.elves.contains(&ne_translation)
            && !self.elves.contains(&nw_translation)
        {
            proposals[0] = Some(n_translation);
            num_proposals += 1;
        }

        if !self.elves.contains(&s_translation)
            && !self.elves.contains(&se_translation)
            && !self.elves.contains(&sw_translation)
        {
            proposals[1] = Some(s_translation);
            num_proposals += 1;
        }

        if !self.elves.contains(&w_translation)
            && !self.elves.contains(&sw_translation)
            && !self.elves.contains(&nw_translation)
        {
            proposals[2] = Some(w_translation);
            num_proposals += 1;
        }

        if !self.elves.contains(&e_translation)
            && !self.elves.contains(&se_translation)
            && !self.elves.contains(&ne_translation)
        {
            proposals[3] = Some(e_translation);
            num_proposals += 1;
        }

        // if all paths are blocked or no paths are blocked, the elf should not move
        if num_proposals == 0 || num_proposals == 4 {
            None
        } else {
            for i in 0..4 {
                let proposal = proposals[(round + i) % 4];
                if proposal.is_some() {
                    return proposal;
                }
            }
            None
        }
    }

    fn play_round(&mut self, round: usize) -> bool {
        let mut moves = HashMap::new();

        for elf in self.elves.iter() {
            if let Some(new_position) = self.propose_elf(elf, round) {
                moves
                    .entry(new_position)
                    .or_insert(Vec::new())
                    .push(elf.clone());
            }
        }

        let mut changed = false;
        for (new_position, old_position) in
            moves
                .into_iter()
                .filter_map(|(new_position, old_positions)| {
                    if old_positions.len() != 1 {
                        None
                    } else {
                        Some((new_position, old_positions[0]))
                    }
                })
        {
            self.elves.remove(&old_position);
            self.elves.insert(new_position);
            changed = true;
        }
        changed
    }

    pub fn compute_ground_tiles(&mut self, num_rounds: usize) -> u32 {
        for round in 0..num_rounds {
            if !self.play_round(round) {
                break;
            }
        }

        let min_x = self.elves.iter().map(|e| e.x).min().unwrap_or(0);
        let max_x = self.elves.iter().map(|e| e.x).max().unwrap_or(0);
        let min_y = self.elves.iter().map(|e| e.y).min().unwrap_or(0);
        let max_y = self.elves.iter().map(|e| e.y).max().unwrap_or(0);
        let num_elves = self.elves.len() as isize;

        (((max_x - min_x + 1) * (max_y - min_y + 1)) - num_elves) as u32
    }

    pub fn num_rounds_until_no_movement(&mut self) -> usize {
        for round in 0..usize::MAX {
            if !self.play_round(round) {
                return round + 1;
            }
        }
        unreachable!()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves: HashSet<Coordinate> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Coordinate::new(x as isize, y as isize))
                    } else {
                        None
                    }
                })
            })
            .collect();

        Ok(Self { elves })
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    input
        .parse::<Grid>()
        .unwrap()
        .compute_ground_tiles(NUM_ROUNDS)
}

fn problem2(input: &str) -> usize {
    input
        .parse::<Grid>()
        .unwrap()
        .num_rounds_until_no_movement()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 110);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 20);
}
