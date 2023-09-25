use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Blueprint {
    pub number: usize,
    pub ore_robot: usize,               // ore
    pub clay_robot: usize,              // ore
    pub obsidian_robot: (usize, usize), // ore, clay
    pub geode_robot: (usize, usize),    // ore, obsidian
}

#[derive(Debug, PartialEq)]
pub struct ParseBlueprintError;

impl FromStr for Blueprint {
    type Err = ParseBlueprintError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BLUEPRINT_REGEX: Regex =
                Regex::new(r"\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)").unwrap();
        }

        let captures = BLUEPRINT_REGEX.captures(&s).ok_or(ParseBlueprintError)?;

        let number = |n| -> Result<usize, _> {
            Ok(captures
                .get(n)
                .ok_or(ParseBlueprintError)?
                .as_str()
                .parse()
                .map_err(|_| ParseBlueprintError)?)
        };

        Ok(Blueprint {
            number: number(1)?,
            ore_robot: number(2)?,
            clay_robot: number(3)?,
            obsidian_robot: (number(4)?, number(5)?),
            geode_robot: (number(6)?, number(7)?),
        })
    }
}

pub struct State {
    pub blueprint: Blueprint,
    pub minutes_elapsed: usize,
    pub ore_robots: usize,
    pub clay_robots: usize,
    pub obsidian_robots: usize,
    pub geode_robots: usize,
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
    pub geodes: usize,
}

impl State {
    pub fn new(blueprint: Blueprint) -> Self {
        Self {
            blueprint,
            minutes_elapsed: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }

    pub fn maybe_build_geode_robot(&mut self) {
        // probaly apply some rule here
        let (needed_ore, needed_obsidian) = self.blueprint.geode_robot;
        if self.ore >= needed_ore && self.obsidian >= needed_obsidian {
            println!("building geode robot");
            self.ore -= needed_ore;
            self.obsidian -= needed_obsidian;
            self.geode_robots += 1;
        }
    }

    pub fn maybe_build_obsidian_robot(&mut self) {
        // probaly apply some rule here
        let (needed_ore, needed_clay) = self.blueprint.obsidian_robot;
        if self.ore >= needed_ore && self.clay >= needed_clay {
            println!("building obsidian robot");
            self.ore -= needed_ore;
            self.clay -= needed_clay;
            self.obsidian_robots += 1;
        }
    }

    pub fn maybe_build_clay_robot(&mut self) {
        // probaly apply some rule here
        let needed_ore = self.blueprint.clay_robot;
        if self.ore >= needed_ore {
            println!("building clay robot");
            self.ore -= needed_ore;
            self.clay_robots += 1;
        }
    }

    pub fn maybe_build_ore_robot(&mut self) {
        // probaly apply some rule here
        let needed_ore = self.blueprint.ore_robot;
        if self.ore >= needed_ore {
            println!("building ore robot");
            self.ore -= needed_ore;
            self.ore_robots += 1;
        }
    }

    pub fn mine_materials(&mut self) {
        if self.ore_robots > 0 {
            println!("mining {} ore", self.ore_robots);
        }
        if self.clay_robots > 0 {
            println!("mining {} clay", self.clay_robots);
        }
        if self.obsidian_robots > 0 {
            println!("mining {} obsidian", self.obsidian_robots);
        }
        if self.geode_robots > 0 {
            println!("cracking {} geodes", self.geode_robots);
        }
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }

    pub fn maybe_build_robots(&mut self) {
        self.maybe_build_geode_robot();
        self.maybe_build_obsidian_robot();
        self.maybe_build_clay_robot();
        self.maybe_build_ore_robot();
    }

    // advance time by one minute
    // mine materials
    // build robots
    pub fn work(&mut self) {
        self.minutes_elapsed += 1;
        println!("minute {}", self.minutes_elapsed);
        self.mine_materials();

        println!(
            "ore: {}, clay: {}, obsidian: {}",
            self.ore, self.clay, self.obsidian
        );

        self.maybe_build_robots();
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blueprint_from_str() {
        let line = "\
            Blueprint 1: \
            Each ore robot costs 3 ore. \
            Each clay robot costs 4 ore. \
            Each obsidian robot costs 4 ore and 18 clay. \
            Each geode robot costs 3 ore and 13 obsidian.";

        let expected = Blueprint {
            number: 1,
            ore_robot: 3,
            clay_robot: 4,
            obsidian_robot: (4, 18),
            geode_robot: (3, 13),
        };

        assert_eq!(line.parse(), Ok(expected));
    }

    #[test]
    fn test_factory() {
        let line = "\
            Blueprint 1: \
            Each ore robot costs 4 ore. \
            Each clay robot costs 2 ore. \
            Each obsidian robot costs 3 ore and 14 clay. \
            Each geode robot costs 2 ore and 7 obsidian.";

        let mut state = State::new(line.parse().unwrap());

        while state.minutes_elapsed < 24 {
            state.work();
        }

        assert_eq!(state.geodes, 9);
    }
}
