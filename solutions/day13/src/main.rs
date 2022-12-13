use std::str::{FromStr, Split};

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

#[derive(Debug)]
enum Packet {
    Num(u32),
    Nested(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.is_right_order_helper(&other) {
            Some(true) => std::cmp::Ordering::Less,
            Some(false) => std::cmp::Ordering::Greater,
            None => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Packet {}

impl Packet {
    fn process_list(tokens: &mut Split<&str>) -> Self {
        let mut v: Vec<Self> = Vec::new();
        while let Some(token) = match tokens.next() {
            Some("]") => None,
            c => c,
        } {
            let val = match token {
                "[" => Self::process_list(tokens),
                "" => continue,
                t => Packet::Num(t.parse().unwrap()),
            };
            v.push(val);
        }
        Self::Nested(v)
    }

    fn len(&self) -> usize {
        match &self {
            Self::Num(_) => 1,
            Self::Nested(v) => v.len(),
        }
    }

    fn is_right_order(&self, other: &Packet) -> bool {
        self.is_right_order_helper(other).unwrap_or(true)
    }

    fn is_right_order_helper(&self, other: &Packet) -> Option<bool> {
        match (&self, &other) {
            (Self::Num(p1), Self::Num(p2)) => {
                if p1 == p2 {
                    None
                } else {
                    Some(p1 < p2)
                }
            }

            (Self::Nested(v1), Self::Nested(v2)) => {
                match v1
                    .iter()
                    .zip(v2.iter())
                    .find(|(p1, p2)| p1.is_right_order_helper(p2).is_some())
                {
                    Some((p1, p2)) => p1.is_right_order_helper(p2),
                    None => {
                        if self.len() == other.len() {
                            None
                        } else {
                            Some(self.len() < other.len())
                        }
                    }
                }
            }
            (Self::Num(p), _) => Self::Nested(vec![Packet::Num(*p)]).is_right_order_helper(other),
            (_, Self::Num(p)) => self.is_right_order_helper(&Self::Nested(vec![Packet::Num(*p)])),
        }
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("[", ",[,").replace("]", ",],");
        let mut tokens = s.split(",");
        tokens.next();
        match tokens.next() {
            None => Ok(Self::Nested(Vec::new())),
            _ => Ok(Self::process_list(&mut tokens)),
        }
    }
}

fn problem1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|line| {
            let (p1, p2) = line.split_once("\n").unwrap();
            (
                p1.trim().parse::<Packet>().unwrap(),
                p2.trim().parse::<Packet>().unwrap(),
            )
        })
        .enumerate()
        .map(|(i, (p1, p2))| {
            if p1.is_right_order(&p2) {
                // println!("{} is in the right order", i + 1);
                (i + 1) as u32
            } else {
                // println!("{} is NOT in the right order", i + 1);
                0 as u32
            }
        })
        .sum()
    // 224 is too low
}

fn problem2(input: &str) -> u32 {
    let mut packets: Vec<Packet> = input
        .split("\n\n")
        .flat_map(|line| {
            let (p1, p2) = line.split_once("\n").unwrap();
            vec![
                p1.trim().parse::<Packet>().unwrap(),
                p2.trim().parse::<Packet>().unwrap(),
            ]
        })
        .collect();
    let divider2_str = "[[2]]";
    let divider6_str = "[[6]]";
    packets.push(divider2_str.parse().unwrap());
    packets.push(divider6_str.parse().unwrap());
    packets.sort();

    let divider2 = divider2_str.parse().unwrap();
    let divider6 = divider6_str.parse().unwrap();

    ((packets.iter().position(|p| p == &divider2).unwrap() + 1)
        * (packets.iter().position(|p| p == &divider6).unwrap() + 1)) as u32
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 13);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 140);
}
