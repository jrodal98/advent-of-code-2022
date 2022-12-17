use std::{cell::RefCell, collections::HashMap, rc::Rc};

const PART1_TIME: u8 = 30;
const PART2_TIME: u8 = 26;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

struct Valve<'a> {
    name: &'a str,
    flow_rate: u16,
    neighbors: Vec<Rc<RefCell<Valve<'a>>>>,
}

impl<'a> Valve<'a> {
    fn from_name(name: &'a str) -> Self {
        Self {
            name,
            flow_rate: 0,
            neighbors: Vec::new(),
        }
    }

    fn from_input(input: &str) -> (Rc<RefCell<Valve>>, usize) {
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
        (
            valves.get("AA").unwrap().clone(),
            valves.values().filter(|v| v.borrow().flow_rate > 0).count(),
        )
    }
}

fn part1_optimal_pressure<'a>(
    valve: Rc<RefCell<Valve<'a>>>,
    total_valves: usize,
    time_remaining: u8,
    // use vec instead of set so I can hash it
    mut opened_valves: Vec<&'a str>,
    cache: &mut HashMap<(&'a str, u8, Vec<&'a str>), u16>,
) -> u16 {
    if opened_valves.len() == total_valves {
        return 0;
    }
    if time_remaining <= 1 {
        return 0;
    }

    let current = valve.borrow();

    let cache_key = (current.name, time_remaining, opened_valves.clone());
    if let Some(cached_result) = cache.get(&cache_key) {
        return *cached_result;
    }

    let mut greatest_potential = 0;
    // do not open valve
    for neighbor in current.neighbors.iter() {
        greatest_potential = greatest_potential.max(part1_optimal_pressure(
            neighbor.clone(),
            total_valves,
            time_remaining - 1,
            opened_valves.clone(),
            cache,
        ));
    }

    // open valve
    let flow = (time_remaining - 1) as u16 * current.flow_rate;
    if flow > 0 && !opened_valves.contains(&current.name) {
        opened_valves.push(current.name);
        opened_valves.sort();
        for neighbor in current.neighbors.iter() {
            greatest_potential = greatest_potential.max(
                flow + part1_optimal_pressure(
                    neighbor.clone(),
                    total_valves,
                    time_remaining - 2,
                    opened_valves.clone(),
                    cache,
                ),
            );
        }
    }

    cache.insert(cache_key, greatest_potential);
    greatest_potential
}

fn part2_optimal_pressure<'a>(
    aa_valve: Rc<RefCell<Valve<'a>>>,
    valve: Rc<RefCell<Valve<'a>>>,
    total_valves: usize,
    time_remaining: u8,
    // use vec instead of set so I can hash it
    mut opened_valves: Vec<&'a str>,
    cache: &mut HashMap<(&'a str, u8, Vec<&'a str>), u16>,
    part1_cache: &mut HashMap<(&'a str, u8, Vec<&'a str>), u16>,
) -> u16 {
    if opened_valves.len() == total_valves {
        return 0;
    }

    let current = valve.borrow();

    let cache_key = (current.name, time_remaining, opened_valves.clone());
    if let Some(cached_result) = cache.get(&cache_key) {
        return *cached_result;
    }

    if time_remaining <= 1 {
        let res = part1_optimal_pressure(
            aa_valve.clone(),
            total_valves,
            PART2_TIME,
            opened_valves,
            part1_cache,
        );
        cache.insert(cache_key, res);
        return res;
    }

    let mut greatest_potential = 0;
    // do not open valve
    for neighbor in current.neighbors.iter() {
        greatest_potential = greatest_potential.max(part2_optimal_pressure(
            aa_valve.clone(),
            neighbor.clone(),
            total_valves,
            time_remaining - 1,
            opened_valves.clone(),
            cache,
            part1_cache,
        ));
    }

    // open valve
    let flow = (time_remaining - 1) as u16 * current.flow_rate;
    if flow > 0 && !opened_valves.contains(&current.name) {
        opened_valves.push(current.name);
        opened_valves.sort();

        for neighbor in current.neighbors.iter() {
            greatest_potential = greatest_potential.max(
                flow + part2_optimal_pressure(
                    aa_valve.clone(),
                    neighbor.clone(),
                    total_valves,
                    time_remaining - 2,
                    opened_valves.clone(),
                    cache,
                    part1_cache,
                ),
            );
        }
    }

    cache.insert(cache_key, greatest_potential);
    greatest_potential
}

fn problem1(input: &str) -> u16 {
    let (start_valve, total_valves) = Valve::from_input(input);
    part1_optimal_pressure(
        start_valve,
        total_valves,
        PART1_TIME,
        Vec::new(),
        &mut HashMap::new(),
    )
}

fn problem2(input: &str) -> u16 {
    let (start_valve, total_valves) = Valve::from_input(input);
    part2_optimal_pressure(
        start_valve.clone(),
        start_valve.clone(),
        total_valves,
        PART2_TIME,
        Vec::new(),
        &mut HashMap::new(),
        &mut HashMap::new(),
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
