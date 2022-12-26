// Advent of Code 2022: Day 16
// https://adventofcode.com/2022/day/16
// Usage: `cargo run <input-file>`

use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

const TIME_LIMIT: usize = 30;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let valves = parse_input(&input);

    println!(
        "The max pressure released after 30 seconds is: {}",
        part_1(&valves)
    );
}

fn part_1(valves: &HashMap<&str, Valve>) -> usize {
    let mut best_total_pressures: HashMap<String, usize> = HashMap::new();
    let mut queue: VecDeque<State> = VecDeque::new();

    let starting_valve = valves.get(&"AA").unwrap();

    queue.push_back(State::new(
        starting_valve.id,
        HashSet::new(),
        HashSet::new(),
        0,
        0,
    ));

    while let Some(state) = queue.pop_front() {
        for next_state in next_states(&valves, &state) {
            let best_pressure = best_total_pressures
                .entry(next_state.hash_key())
                .or_insert(0);

            if next_state.total_pressure > *best_pressure {
                *best_pressure = next_state.total_pressure;
                queue.push_back(next_state);
            }
        }
    }

    best_total_pressures.into_values().max().unwrap()
}

// closed valves that you can reach under the time limit from the current state
pub fn next_states<'a>(valves: &HashMap<&str, Valve<'a>>, state: &State<'a>) -> Vec<State<'a>> {
    // if we can't navigate to an adjacent valve and open it by minute 29
    // then there are no viable next states from this state
    if state.minutes_elapsed > TIME_LIMIT - 3 {
        return vec![];
    }

    let mut next_states: Vec<State> = vec![];
    // (valve_id, minutes elapsed to get to this valve, valves visited in path)
    let mut valves_to_check: VecDeque<(&'a str, usize, Vec<&'a str>)> = VecDeque::new();
    let current_valve = valves.get(state.valve_id).unwrap();

    // the current valve is open (or we're choosing not to open it?) and the minutes elapsed is
    // the time after it was opened
    // now what we want to check is the adjacent valves, so we push those on the queue
    // and the time for each is the time we would ARRIVE at that valve (not including any opening
    // time)
    for id in &current_valve.next_valves {
        valves_to_check.push_back((&id, state.minutes_elapsed + 1, vec![current_valve.id]));
    }

    while let Some((id, minutes, valves_in_path)) = valves_to_check.pop_front() {
        let valve = valves.get(id).unwrap();
        if valve.flow_rate == 0 || state.open_valves.contains(&id) {
            // this valve is open, so we want to check valves we can get to from
            // here, and it will take 1 minute to go to the next valve
            // and then it would take two minutes to open it, so skip ones we couldn't open
            if minutes + 2 < TIME_LIMIT {
                for id in valve
                    .next_valves
                    .iter()
                    .filter(|v| !valves_in_path.contains(*v))
                {
                    let mut valves_in_path = valves_in_path.clone();
                    valves_in_path.push(id);
                    valves_to_check.push_back((&id, minutes + 1, valves_in_path));
                }
            }
        } else {
            // there are two options when we encounter a closed valve,
            // we can open it, or we can keep going in search of better options
            // let's push both options on to the next_states queue

            // add the valves in the path to this node to the list of visited valves
            let mut visited_valves = state.visited_valves.clone();
            for valve_in_path in valves_in_path {
                if !state.open_valves.contains(valve_in_path) {
                    visited_valves.insert(valve_in_path);
                }
            }

            // don't open the valve even though it's closed, just navigate to it
            // we don't need to consider this unless we'd still be able to get to another
            // closed valve and open it by minute 29
            if minutes + 2 < TIME_LIMIT {
                visited_valves.insert(id);
                next_states.push(State::new(
                    id,
                    state.open_valves.clone(),
                    visited_valves.clone(),
                    state.total_pressure,
                    minutes,
                ));
            }

            // navigate to the valve and open it
            // it takes one minute to open the valve, so only consider this if we have time
            if minutes + 1 < TIME_LIMIT {
                let mut open_valves = state.open_valves.clone();
                open_valves.insert(id);

                let minutes_elapsed = minutes + 1;

                let total_pressure =
                    state.total_pressure + valve.flow_rate * (TIME_LIMIT - minutes_elapsed);

                next_states.push(State::new(
                    id,
                    open_valves,
                    visited_valves,
                    total_pressure,
                    minutes_elapsed,
                ))
            }
        }
    }

    next_states
}

#[derive(Debug)]
pub struct State<'a> {
    pub valve_id: &'a str,
    pub open_valves: HashSet<&'a str>,
    pub visited_valves: HashSet<&'a str>,
    pub total_pressure: usize,
    pub minutes_elapsed: usize,
}

impl<'a> State<'a> {
    pub fn new(
        valve_id: &'a str,
        open_valves: HashSet<&'a str>,
        visited_valves: HashSet<&'a str>,
        total_pressure: usize,
        minutes_elapsed: usize,
    ) -> Self {
        Self {
            valve_id,
            open_valves,
            visited_valves,
            total_pressure,
            minutes_elapsed,
        }
    }

    pub fn hash_key(&self) -> String {
        let mut open_valves: Vec<_> = self.open_valves.iter().cloned().collect();
        let mut visited_valves: Vec<_> = self.visited_valves.iter().cloned().collect();
        open_valves.sort();
        visited_valves.sort();
        format!("{}-{}", open_valves.join(""), visited_valves.join(""))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Valve<'a> {
    pub id: &'a str,
    pub flow_rate: usize,
    pub next_valves: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    pub fn new(id: &'a str, flow_rate: usize, next_valves: Vec<&'a str>) -> Self {
        Self {
            id,
            flow_rate,
            next_valves,
        }
    }
}

fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, Valve<'a>> {
    let pattern = r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)";
    let re = Regex::new(&pattern).unwrap();

    input
        .trim()
        .split('\n')
        .map(|line| {
            let captures = re.captures(&line).unwrap();
            let id = captures.get(1).unwrap().as_str();
            let flow_rate = captures.get(2).unwrap().as_str().parse().unwrap();
            let next_valves = captures
                .get(3)
                .unwrap()
                .as_str()
                .trim()
                .split(", ")
                .collect();

            (id, Valve::new(id, flow_rate, next_valves))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let valves = parse_input(&input);
        assert_eq!(valves.len(), 10);
        assert_eq!(
            valves.get("AA"),
            Some(&Valve::new("AA", 0, vec!["DD", "II", "BB"]))
        );
        assert_eq!(valves.get("JJ"), Some(&Valve::new("JJ", 21, vec!["II"])));
    }

    // #[test]
    // fn test_next_states() {
    //     let input = fs::read_to_string("test-input.txt").unwrap();
    //     let valves = parse_input(&input);

    //     let starting_valve = valves.get(&"AA").unwrap();
    //     let aa = State::new(
    //         starting_valve.id,
    //         HashSet::new(),
    //         HashSet::new(),
    //         0,
    //         0,
    //         vec![],
    //     );
    //     let dd_open = &next_states(&valves, &aa)[1];
    //     let cc = &next_states(&valves, &dd_open)[0];
    //     let bb_open = &next_states(&valves, &cc)[1];
    //     println!("bb open: {:?}", bb_open);
    //     for state in next_states(&valves, &bb_open) {
    //         println!("{:?}", state)
    //     }
    //     assert!(false);
    // }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let valves = parse_input(&input);
        assert_eq!(part_1(&valves), 1651);
    }
}
