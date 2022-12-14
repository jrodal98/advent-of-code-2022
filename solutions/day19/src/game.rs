use std::{collections::HashMap, str::FromStr};

use crate::factory::{Blueprint, Factory, FactoryOutput, Robot};

const PART1_TIME: usize = 24;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub struct GameState {
    factory: Factory,
    resources: FactoryOutput,
    minutes_passed: u16,
    incoming_robot: Option<Robot>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Game {
    blueprint: Blueprint,
    state: GameState,
}

impl Game {
    pub fn max_num_geodes(&self, time: usize) -> u16 {
        let mut max_seen: Vec<u16> = vec![0; time + 1];
        let optimal = Self::find_optimal(self.clone(), time, &mut HashMap::new(), &mut max_seen);
        println!("Id: {}, geodes: {}", self.blueprint.id, optimal);
        optimal
    }
    pub fn quality_level(&self) -> u16 {
        self.blueprint.id * self.max_num_geodes(PART1_TIME)
    }

    fn should_produce_ore(&self) -> bool {
        self.state.factory.ore_robots < self.blueprint.max_ore() 
    }

    fn should_produce_clay(&self) -> bool {
        self.state.factory.clay_robots < self.blueprint.max_clay() 
    }

    fn should_produce_obsidian(&self) -> bool {
        self.state.factory.obsidian_robots < self.blueprint.max_obsidian() 
    }

    fn collect_resources(&mut self) {
        self.state.resources += self.state.factory.produce();

        // throw away extra resources (for better caching)
        if !self.should_produce_ore() {
            self.state.resources.ore = self.state.resources.ore.min(self.blueprint.max_ore());
        }

        if !self.should_produce_clay() {
            self.state.resources.clay = self.state.resources.clay.min(self.blueprint.max_clay());
        }

        if !self.should_produce_obsidian() {
            self.state.resources.obsidian = self.state.resources.obsidian.min(self.blueprint.max_obsidian());
        }

    }

    fn deliver_robot(&mut self) {
        match self.state.incoming_robot {
            None => (),
            Some(Robot::Ore) => self.state.factory.ore_robots += 1,
            Some(Robot::Clay) => self.state.factory.clay_robots += 1,
            Some(Robot::Obsidian) => self.state.factory.obsidian_robots += 1,
            Some(Robot::Geode) => self.state.factory.geode_robots += 1,
        }
        self.state.incoming_robot = None;
    }

    fn find_optimal(
        mut game: Self,
        time_remaining: usize,
        cache: &mut HashMap<GameState, u16>,
        max_seen: &mut Vec<u16>,
    ) -> u16 {
        game.state.minutes_passed += 1;
        let key = game.state.clone();

        if let Some(cached_result) = cache.get(&game.state) {
            return *cached_result;
        }

        game.deliver_robot();
        let mut optimal: Option<u16> = None;
        if time_remaining == 0 {
            optimal = Some(game.state.resources.geode);
        } else if time_remaining == 1 {
            game.collect_resources();
            optimal = Some(game.state.resources.geode);
        } else if game.state.resources.can_afford(&game.blueprint.geode_cost) {
            game.buy_robot(Robot::Geode);
            game.collect_resources();
            optimal = Some(Self::find_optimal(
                game,
                time_remaining - 1,
                cache,
                max_seen,
            ));
        }

        let highest_possible_geode = game.state.resources.geode as usize + game.state.factory.geode_robots as usize * time_remaining
                    + ((time_remaining * (time_remaining + 1))
                    / 2);

        if optimal.is_some() 
            // cannot possibly beat max
        || highest_possible_geode
                < max_seen[time_remaining] as usize
        {
            let optimal = optimal.unwrap_or(0);
            max_seen[time_remaining] = max_seen[time_remaining].max(optimal);
            cache.insert(key, optimal);
            return optimal;
        }

        let mut optimal = 0;

        // don't bother generating ore factories if you can't spend it fast enough
        if game.should_produce_ore() && game.state.resources.can_afford(&game.blueprint.ore_cost) {
            let mut ore_game = game.clone();
            ore_game.buy_robot(Robot::Ore);
            ore_game.collect_resources();
            optimal = optimal.max(Self::find_optimal(
                ore_game,
                time_remaining - 1,
                cache,
                max_seen,
            ));
        }

        // don't bother generating clay factories if you can't spend it fast enough
        if game.should_produce_clay() && game.state.resources.can_afford(&game.blueprint.clay_cost) {
            let mut clay_game = game.clone();
            clay_game.buy_robot(Robot::Clay);
            clay_game.collect_resources();
            optimal = optimal.max(Self::find_optimal(
                clay_game,
                time_remaining - 1,
                cache,
                max_seen,
            ));
        }

        // don't bother generating obsidian factories if you can't spend it fast enough
        if game.should_produce_obsidian() && game
            .state
            .resources
            .can_afford(&game.blueprint.obsidian_cost)
        {
            let mut obsidian_game = game.clone();
            obsidian_game.buy_robot(Robot::Obsidian);
            obsidian_game.collect_resources();
            optimal = optimal.max(Self::find_optimal(
                obsidian_game,
                time_remaining - 1,
                cache,
                max_seen,
            ));
        }

        // do nothing
        game.collect_resources();
        optimal = optimal.max(Self::find_optimal(
            game,
            time_remaining - 1,
            cache,
            max_seen,
        ));

        max_seen[time_remaining] = max_seen[time_remaining].max(optimal);
        cache.insert(key, optimal);
        optimal
    }

    fn buy_robot(&mut self, robot: Robot) {
        match robot {
            Robot::Ore => {
                self.state.resources -= self.blueprint.ore_cost;
            }
            Robot::Clay => {
                self.state.resources -= self.blueprint.clay_cost;
            }
            Robot::Obsidian => {
                self.state.resources -= self.blueprint.obsidian_cost;
            }
            Robot::Geode => {
                self.state.resources -= self.blueprint.geode_cost;
            }
        }
        self.state.incoming_robot = Some(robot);
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) =
            sscanf::sscanf!(s,"Blueprint {u16}: Each ore robot costs {u16} ore. Each clay robot costs {u16} ore. Each obsidian robot costs {u16} ore and {u16} clay. Each geode robot costs {u16} ore and {u16} obsidian.").unwrap();

        let ore_cost = FactoryOutput::default().with_ore(ore_ore);
        let clay_cost = FactoryOutput::default().with_ore(clay_ore);
        let obsidian_cost = FactoryOutput::default()
            .with_ore(obsidian_ore)
            .with_clay(obsidian_clay);
        let geode_cost = FactoryOutput::default()
            .with_ore(geode_ore)
            .with_obsidian(geode_obsidian);

        let blueprint = Blueprint {
            id,
            ore_cost,
            clay_cost,
            obsidian_cost,
            geode_cost,
        };

        let state = GameState::default();
        Ok(Self { blueprint, state })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_from_str() {
        let blueprint_str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        let actual: Game = blueprint_str.parse().unwrap();
        let expected = Game {
            blueprint: Blueprint {
                id: 1,
                ore_cost: FactoryOutput {
                    ore: 4,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                clay_cost: FactoryOutput {
                    ore: 2,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                obsidian_cost: FactoryOutput {
                    ore: 3,
                    clay: 14,
                    obsidian: 0,
                    geode: 0,
                },
                geode_cost: FactoryOutput {
                    ore: 2,
                    clay: 0,
                    obsidian: 7,
                    geode: 0,
                },
            },
            state: GameState {
                factory: Factory {
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                },
                resources: FactoryOutput {
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                minutes_passed: 0,
                incoming_robot: None,
            },
        };

        assert_eq!(expected, actual);
    }
}
