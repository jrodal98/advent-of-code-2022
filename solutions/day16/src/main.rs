use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
    cache: &mut HashMap<(String, u32, Vec<String>), u32>,
) -> u32 {
    if time_remaining <= 1 {
        return 0;
    }

    let current = valve.borrow();

    let cache_key = (current.name.clone(), time_remaining, opened_valves.clone());
    if let Some(cached_result) = cache.get(&cache_key) {
        return *cached_result;
    }

    let mut paths: Vec<u32> = Vec::new();
    // do not open valve
    for neighbor in current.neighbors.iter() {
        paths.push(part1_optimal_pressure(
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
            paths.push(
                flow + part1_optimal_pressure(
                    neighbor.clone(),
                    time_remaining - 2,
                    opened_valves.clone(),
                    cache,
                ),
            );
        }
    }

    let res = paths.into_iter().max().unwrap();
    cache.insert(cache_key, res);
    res
}

fn part2_optimal_pressure(
    my_valve: Rc<RefCell<Valve>>,
    elephant_valve: Rc<RefCell<Valve>>,
    time_remaining: u32,
    // use vec instead of set so I can hash it
    mut opened_valves: Vec<String>,
    cache: &mut HashMap<(String, String, u32, Vec<String>), u32>,
) -> u32 {
    if time_remaining <= 1 {
        return 0;
    }

    let my_current = my_valve.borrow();
    let elephant_current = elephant_valve.borrow();

    let cache_key = (
        my_current.name.clone(),
        elephant_current.name.clone(),
        time_remaining,
        opened_valves.clone(),
    );
    if let Some(cached_result) = cache.get(&cache_key) {
        return *cached_result;
    }

    let mut paths: Vec<u32> = Vec::new();

    // we both move
    for my_neighbor in my_current.neighbors.iter() {
        for elephant_neighbor in elephant_current.neighbors.iter() {
            paths.push(part2_optimal_pressure(
                my_neighbor.clone(),
                elephant_neighbor.clone(),
                time_remaining - 1,
                opened_valves.clone(),
                cache,
            ));
        }
    }

    // I move, elephant opens valve
    let elephant_flow = (time_remaining - 1) * elephant_current.flow_rate;
    if elephant_flow > 0 && !opened_valves.contains(&elephant_current.name) {
        let mut opened_valves_with_elephant = opened_valves.clone();
        opened_valves_with_elephant.push(elephant_current.name.clone());

        for my_neighbor in my_current.neighbors.iter() {
            paths.push(
                elephant_flow
                    + part2_optimal_pressure(
                        my_neighbor.clone(),
                        elephant_valve.clone(),
                        time_remaining - 1,
                        opened_valves_with_elephant.clone(),
                        cache,
                    ),
            );
        }
    }

    // elephant moves, I open valve
    let my_flow = (time_remaining - 1) * my_current.flow_rate;
    if my_flow > 0 && !opened_valves.contains(&my_current.name) {
        let mut opened_valves_with_me = opened_valves.clone();
        opened_valves_with_me.push(my_current.name.clone());

        for elephant_neighbor in elephant_current.neighbors.iter() {
            paths.push(
                my_flow
                    + part2_optimal_pressure(
                        my_valve.clone(),
                        elephant_neighbor.clone(),
                        time_remaining - 1,
                        opened_valves_with_me.clone(),
                        cache,
                    ),
            );
        }
    }

    // we both open valve
    if my_flow > 0
        && elephant_flow > 0
        && my_current.name != elephant_current.name
        && !opened_valves.contains(&my_current.name)
        && !opened_valves.contains(&elephant_current.name)
    {
        opened_valves.push(my_current.name.clone());
        opened_valves.push(elephant_current.name.clone());

        paths.push(
            my_flow
                + elephant_flow
                + part2_optimal_pressure(
                    my_valve.clone(),
                    elephant_valve.clone(),
                    time_remaining - 1,
                    opened_valves,
                    cache,
                ),
        );
    }

    let res = paths.into_iter().max().unwrap();
    cache.insert(cache_key, res);
    res
}

fn problem1(input: &str) -> u32 {
    let start_valve = Valve::from_input(input);
    part1_optimal_pressure(start_valve, PART1_TIME, Vec::new(), &mut HashMap::new())
}

fn problem2(input: &str) -> u32 {
    let start_valve = Valve::from_input(input);
    part2_optimal_pressure(
        start_valve.clone(),
        start_valve.clone(),
        PART2_TIME,
        Vec::new(),
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
