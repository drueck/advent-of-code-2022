// Advent of Code 2022: Day 16
// https://adventofcode.com/2022/day/16
// Usage: `cargo run <input-file>`

use itertools::iproduct;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;
use Action::{Open, Visit, Wait};
use Strategy::{Alone, WithElephant};

const TIME_LIMIT: usize = 30;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let valves = parse_input(&input);

    println!(
        "The max pressure released with you working alone is: {}",
        find_best_pressure(&valves, Alone)
    );

    println!(
        "The max pressure released with you and the elephant working together is: {}",
        find_best_pressure(&valves, WithElephant)
    );
}

pub enum Strategy {
    Alone,
    WithElephant,
}

fn find_best_pressure(valves: &HashMap<&str, Valve>, strategy: Strategy) -> usize {
    let mut best_total_pressures: HashMap<String, usize> = HashMap::new();
    let mut queue: VecDeque<GlobalState> = VecDeque::new();

    let starting_valve = valves.get(&"AA").unwrap();
    let valves_worth_opening = valves
        .iter()
        .filter(|(_, valve)| valve.flow_rate > 0)
        .count();

    let (human_state, elephant_state) = match strategy {
        Alone => (WorkerState::new(starting_valve.id, 0), None),
        WithElephant => (
            WorkerState::new(starting_valve.id, 4),
            Some(WorkerState::new(starting_valve.id, 4)),
        ),
    };

    queue.push_back(GlobalState::new(
        human_state,
        elephant_state,
        HashSet::new(),
        HashSet::new(),
        0,
    ));

    while let Some(global_state) = queue.pop_front() {
        for next_state in next_global_states(&valves, &global_state) {
            let best_pressure = best_total_pressures
                .entry(next_state.hash_key())
                .or_insert(0);

            if next_state.total_pressure > *best_pressure {
                *best_pressure = next_state.total_pressure;
                if next_state.open_valves.len() < valves_worth_opening {
                    queue.push_back(next_state);
                }
            }
        }
    }

    best_total_pressures.into_values().max().unwrap()
}

pub fn next_moves<'a>(
    worker_state: &WorkerState<'a>,
    global_state: &GlobalState<'a>,
    valves: &HashMap<&str, Valve<'a>>,
) -> Vec<Move<'a>> {
    if worker_state.minutes_elapsed > TIME_LIMIT - 3 {
        return vec![];
    }

    let mut next_moves: Vec<Move> = vec![];
    // (valve_id, minutes elapsed to get to this valve, valves visited in path)
    let mut valves_to_check: VecDeque<(&'a str, usize, Vec<&'a str>)> = VecDeque::new();
    let current_valve = valves.get(worker_state.valve_id).unwrap();

    for id in &current_valve.next_valves {
        valves_to_check.push_back((
            &id,
            worker_state.minutes_elapsed + 1,
            vec![current_valve.id],
        ));
    }

    while let Some((id, minutes, valves_in_path)) = valves_to_check.pop_front() {
        let valve = valves.get(id).unwrap();
        if valve.flow_rate == 0 || global_state.open_valves.contains(&id) {
            // open or not worth opening
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
            // closed - we have two option here, so add both of them
            // visit
            next_moves.push(Move::new(id, valves_in_path.clone(), Visit, minutes, 0));
            // open
            next_moves.push(Move::new(
                id,
                valves_in_path,
                Open,
                minutes + 1,
                valve.flow_rate * (TIME_LIMIT - minutes - 1),
            ))
        }
    }

    next_moves
}

pub fn next_global_states<'a>(
    valves: &HashMap<&str, Valve<'a>>,
    global_state: &GlobalState<'a>,
) -> Vec<GlobalState<'a>> {
    let mut next_global_states = vec![];

    let next_human_moves = next_moves(&global_state.human_state, &global_state, &valves);
    let next_elephant_moves = match &global_state.elephant_state {
        Some(elephant_state) => next_moves(&elephant_state, &global_state, &valves),
        None => vec![Move::new(&"AA", vec![], Wait, 0, 0)],
    };

    for (human_move, elephant_move) in iproduct!(&next_human_moves, &next_elephant_moves) {
        if human_move.opening_same_valve_as(&elephant_move) {
            continue;
        }
        let mut next_global_state = (*global_state).clone();
        next_global_state.apply_human_move(&human_move);
        next_global_state.apply_elephant_move(&elephant_move);
        next_global_states.push(next_global_state);
    }

    next_global_states
}

#[derive(Clone, Debug)]
pub struct GlobalState<'a> {
    pub human_state: WorkerState<'a>,
    pub elephant_state: Option<WorkerState<'a>>,
    pub open_valves: HashSet<&'a str>,
    pub visited_valves: HashSet<&'a str>,
    pub total_pressure: usize,
}

impl<'a> GlobalState<'a> {
    pub fn new(
        human_state: WorkerState<'a>,
        elephant_state: Option<WorkerState<'a>>,
        open_valves: HashSet<&'a str>,
        visited_valves: HashSet<&'a str>,
        total_pressure: usize,
    ) -> Self {
        Self {
            human_state,
            elephant_state,
            open_valves,
            visited_valves,
            total_pressure,
        }
    }

    pub fn apply_human_move(&mut self, m: &Move<'a>) {
        if m.action != Wait {
            self.human_state = WorkerState::new(m.valve_id, m.minutes_elapsed);
            self.apply_move(&m);
        }
    }

    // just ignore any move if there's no existing elephant state
    // need to improve the ergonomics of this
    pub fn apply_elephant_move(&mut self, m: &Move<'a>) {
        if m.action != Wait {
            self.elephant_state = Some(WorkerState::new(m.valve_id, m.minutes_elapsed));
            self.apply_move(&m);
        }
    }

    fn apply_move(&mut self, m: &Move<'a>) {
        match m.action {
            Visit => {
                self.visited_valves.insert(m.valve_id);
                for v in &m.valves_in_path {
                    if !self.open_valves.contains(v) {
                        self.visited_valves.insert(v);
                    }
                }
            }
            Open => {
                self.open_valves.insert(m.valve_id);
                self.total_pressure += m.pressure_released;
                for v in &m.valves_in_path {
                    if !self.open_valves.contains(v) {
                        self.visited_valves.insert(v);
                    }
                }
            }
            Wait => {}
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

#[derive(Clone, Debug)]
pub struct WorkerState<'a> {
    pub valve_id: &'a str,
    pub minutes_elapsed: usize,
}

impl<'a> WorkerState<'a> {
    pub fn new(valve_id: &'a str, minutes_elapsed: usize) -> Self {
        Self {
            valve_id,
            minutes_elapsed,
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum Action {
    Visit,
    Open,
    Wait,
}

pub struct Move<'a> {
    pub valve_id: &'a str,
    pub valves_in_path: Vec<&'a str>,
    pub action: Action,
    pub minutes_elapsed: usize,
    pub pressure_released: usize,
}

impl<'a> Move<'a> {
    pub fn new(
        valve_id: &'a str,
        valves_in_path: Vec<&'a str>,
        action: Action,
        minutes_elapsed: usize,
        pressure_released: usize,
    ) -> Self {
        Self {
            valve_id,
            valves_in_path,
            action,
            minutes_elapsed,
            pressure_released,
        }
    }

    pub fn opening_same_valve_as(&self, other: &Self) -> bool {
        self.valve_id == other.valve_id && self.action == Open && other.action == Open
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

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let valves = parse_input(&input);
        assert_eq!(find_best_pressure(&valves, Alone), 1651);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let valves = parse_input(&input);
        assert_eq!(find_best_pressure(&valves, WithElephant), 1707);
    }
}
