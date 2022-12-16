use std::{cell::RefCell, collections::HashMap, num::NonZeroUsize, rc::Rc};

use lru::LruCache;

const PART1_TIME: u32 = 30;
const PART2_TIME: u32 = 26;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

struct Valve {
    name: String,
    flow_rate: u32,
    neighbors: Vec<Rc<RefCell<Valve>>>,
}

impl Valve {
    fn from_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            flow_rate: 0,
            neighbors: Vec::new(),
        }
    }

    fn from_input(input: &str) -> Rc<RefCell<Valve>> {
        let mut valves: HashMap<&str, Rc<RefCell<Valve>>> = HashMap::new();
        // set name and flow rate
        for line in input.lines() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let valve_name = tokens[1];
            let flow_rate = tokens[4][5..].strip_suffix(";").unwrap();
            let tunnels: Vec<Rc<RefCell<Valve>>> = tokens[9..]
                .iter()
                .map(|n| n.strip_suffix(",").unwrap_or(n))
                .map(|n| {
                    valves
                        .entry(n)
                        .or_insert(Rc::new(RefCell::new(Valve::from_name(n))))
                        .clone()
                })
                .collect();

            let valve = valves
                .entry(valve_name)
                .or_insert(Rc::new(RefCell::new(Valve::from_name(valve_name))));
            valve.borrow_mut().flow_rate = flow_rate.parse().unwrap();
            valve.borrow_mut().neighbors = tunnels;
        }
        valves.get("AA").unwrap().clone()
    }
}

fn part1_optimal_pressure(
    valve: Rc<RefCell<Valve>>,
    time_remaining: u32,
    // use vec instead of set so I can hash it
    mut opened_valves: Vec<String>,
    cache: &mut LruCache<(String, u32, Vec<String>), u32>,
) -> u32 {
    if time_remaining <= 1 {
        return 0;
    }

    let current = valve.borrow();

    let cache_key = (current.name.clone(), time_remaining, opened_valves.clone());
    if let Some(cached_result) = cache.get(&cache_key) {
        return *cached_result;
    }

    let mut greatest_potential = 0;
    // do not open valve
    for neighbor in current.neighbors.iter() {
        greatest_potential = greatest_potential.max(part1_optimal_pressure(
            neighbor.clone(),
            time_remaining - 1,
            opened_valves.clone(),
            cache,
        ));
    }

    // open valve
    let flow = (time_remaining - 1) * current.flow_rate;
    if flow > 0 && !opened_valves.contains(&current.name) {
        opened_valves.push(current.name.clone());
        for neighbor in current.neighbors.iter() {
            greatest_potential = greatest_potential.max(
                flow + part1_optimal_pressure(
                    neighbor.clone(),
                    time_remaining - 2,
                    opened_valves.clone(),
                    cache,
                ),
            );
        }
    }

    cache.put(cache_key, greatest_potential);
    greatest_potential
}

fn part2_optimal_pressure(
    aa_valve: Rc<RefCell<Valve>>,
    valve: Rc<RefCell<Valve>>,
    time_remaining: u32,
    // use vec instead of set so I can hash it
    mut opened_valves: Vec<String>,
    cache: &mut LruCache<(String, u32, Vec<String>), u32>,
    part1_cache: &mut LruCache<(String, u32, Vec<String>), u32>,
) -> u32 {
    if time_remaining <= 1 {
        return part1_optimal_pressure(aa_valve.clone(), PART2_TIME, opened_valves, part1_cache);
    }

    let current = valve.borrow();

    let cache_key = (current.name.clone(), time_remaining, opened_valves.clone());
    if let Some(cached_result) = cache.get(&cache_key) {
        return *cached_result;
    }

    let mut greatest_potential = 0;
    // do not open valve
    for neighbor in current.neighbors.iter() {
        greatest_potential = greatest_potential.max(part2_optimal_pressure(
            aa_valve.clone(),
            neighbor.clone(),
            time_remaining - 1,
            opened_valves.clone(),
            cache,
            part1_cache,
        ));
    }

    // open valve
    let flow = (time_remaining - 1) * current.flow_rate;
    if flow > 0 && !opened_valves.contains(&current.name) {
        opened_valves.push(current.name.clone());

        for neighbor in current.neighbors.iter() {
            greatest_potential = greatest_potential.max(
                flow + part2_optimal_pressure(
                    aa_valve.clone(),
                    neighbor.clone(),
                    time_remaining - 2,
                    opened_valves.clone(),
                    cache,
                    part1_cache,
                ),
            );
        }
    }

    cache.put(cache_key, greatest_potential);
    greatest_potential
}

fn problem1(input: &str) -> u32 {
    let start_valve = Valve::from_input(input);
    part1_optimal_pressure(
        start_valve,
        PART1_TIME,
        Vec::new(),
        &mut LruCache::new(NonZeroUsize::new(31_000_000).unwrap()),
    )
}

fn problem2(input: &str) -> u32 {
    let start_valve = Valve::from_input(input);
    part2_optimal_pressure(
        start_valve.clone(),
        start_valve.clone(),
        PART2_TIME,
        Vec::new(),
        &mut LruCache::new(NonZeroUsize::new(20_000_000).unwrap()),
        &mut LruCache::new(NonZeroUsize::new(11_000_000).unwrap()),
    )
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 1651);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 1707);
}
