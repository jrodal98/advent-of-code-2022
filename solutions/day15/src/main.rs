use entities::{Coordinate, Sensor};

use crate::entities::MAX_XY;

pub mod entities;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input, 2_000_000));
    println!("Problem 2: {}", problem2(input, MAX_XY));
}

fn get_sensors(input: &str) -> Vec<Sensor> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn problem1(input: &str, y: isize) -> usize {
    let sensors = get_sensors(input);
    // dbg!(&sensors);
    let min_x = sensors.iter().map(|s| s.position.x).min().unwrap();
    let max_x = sensors.iter().map(|s| s.position.x).max().unwrap();
    let mut impossible = 0;
    for x in min_x - y * 10..=max_x + y * 10 {
        for s in sensors.iter() {
            let c = Coordinate { x, y };
            if c != s.beacon && c.manhattan_distance(&s.position) <= s.distance_to_beacon {
                impossible += 1;
                break;
            }
        }
    }
    impossible
}

// The unique beacon must be distance_to_beacon + 1 away from an existing sensor
// if it were +0, it would contradict the "There is never a tie where two beacons are the
// same distance to a sensor" condition
// if it were +2 or higher, then...
// another beacon must already be between those -
// otherwise, it would contradict the uniqueness claim
fn problem2(input: &str, max_xy: isize) -> usize {
    let sensors = get_sensors(input);
    sensors
        .iter()
        .flat_map(|s| {
            let d_to_unique_beacon = (s.distance_to_beacon + 1) as isize;
            (0..=d_to_unique_beacon)
                .flat_map(|dx| {
                    let dy = d_to_unique_beacon - dx;
                    vec![(-dx, -dy), (-dx, dy), (dx, -dy), (dx, dx)]
                        .into_iter()
                        .map(|(x, y)| Coordinate {
                            x: s.position.x - x,
                            y: s.position.y - y,
                        })
                        .filter(|coord| {
                            coord.x >= 0 && coord.x <= max_xy && coord.y >= 0 && coord.y <= max_xy
                        })
                })
                .collect::<Vec<Coordinate>>()
        })
        .find(|coord| {
            sensors
                .iter()
                .all(|sensor| sensor.position.manhattan_distance(coord) > sensor.distance_to_beacon)
        })
        .unwrap()
        .tuning_distance()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input, 10);
    assert_eq!(res, 26);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input, 20);
    assert_eq!(res, 56000011);
}
