#[derive(Debug, PartialEq, Eq, Default)]
pub struct FactoryOutput {
    pub ore: u16,
    pub clay: u16,
    pub obsidian: u16,
    pub geode: u16,
}

impl FactoryOutput {
    pub fn with_ore(mut self, ore: u16) -> Self {
        self.ore = ore;
        self
    }

    pub fn with_clay(mut self, clay: u16) -> Self {
        self.clay = clay;
        self
    }

    pub fn with_obsidian(mut self, obsidian: u16) -> Self {
        self.obsidian = obsidian;
        self
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Factory {
    pub ore_robots: u16,
    pub clay_robots: u16,
    pub obsidian_robots: u16,
    pub geode_robots: u16,
}

impl Factory {
    pub fn produce(&self) -> FactoryOutput {
        FactoryOutput {
            ore: self.ore_robots,
            clay: self.clay_robots,
            obsidian: self.obsidian_robots,
            geode: self.geode_robots,
        }
    }
}

impl Default for Factory {
    fn default() -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Blueprint {
    pub id: u16,
    pub ore_cost: FactoryOutput,
    pub clay_cost: FactoryOutput,
    pub obsidian_cost: FactoryOutput,
    pub geode_cost: FactoryOutput,
}
