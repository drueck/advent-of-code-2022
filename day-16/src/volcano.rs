use crate::moves::{Action, Move};
use crate::state::{GlobalState, WorkerState};
use itertools::iproduct;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use Action::{Open, Visit, Wait};
use Strategy::{Alone, WithElephant};

const TIME_LIMIT: usize = 30;
const MAYBE_SKIP_LIMIT_WITH_ELEPHANT: usize = 4;
const MAYBE_SKIP_LIMIT_ALONE: usize = 5;

pub enum Strategy {
    Alone,
    WithElephant,
}

pub struct Volcano<'a> {
    pub valves: HashMap<&'a str, Valve<'a>>,
}

impl<'a> Volcano<'a> {
    pub fn new(input: &'a str) -> Self {
        let pattern = r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)";
        let re = Regex::new(&pattern).unwrap();

        let valves = input
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
            .collect();

        Self { valves }
    }

    pub fn find_best_pressure(&self, strategy: Strategy) -> usize {
        let mut best_total_pressures: HashMap<String, usize> = HashMap::new();
        let mut queue: VecDeque<GlobalState> = VecDeque::new();

        let starting_valve = self.valves.get(&"AA").unwrap();
        let valves_worth_opening = self
            .valves
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
            for next_state in self.next_global_states(&global_state) {
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

    fn next_global_states(&self, global_state: &GlobalState<'a>) -> Vec<GlobalState<'a>> {
        let next_human_moves = self.next_moves(&global_state.human_state, &global_state);
        let next_elephant_moves = match &global_state.elephant_state {
            Some(elephant_state) => self.next_moves(&elephant_state, &global_state),
            None => vec![Move::new(&"AA", vec![], Wait, 0, 0)],
        };

        if next_human_moves.is_empty() || next_elephant_moves.is_empty() {
            return vec![];
        }

        let mut next_global_states = vec![];

        for (human_move, elephant_move) in iproduct!(&next_human_moves, &next_elephant_moves) {
            if human_move.ineffective_combination_with(&elephant_move) {
                continue;
            }
            let mut next_global_state = (*global_state).clone();
            next_global_state.apply_human_move(&human_move);
            next_global_state.apply_elephant_move(&elephant_move);
            next_global_states.push(next_global_state);
        }

        next_global_states
    }

    fn next_moves(
        &self,
        worker_state: &WorkerState<'a>,
        global_state: &GlobalState<'a>,
    ) -> Vec<Move<'a>> {
        if worker_state.minutes_elapsed > TIME_LIMIT - 3 {
            return vec![];
        }

        let mut next_moves: Vec<Move> = vec![];
        // (valve_id, minutes elapsed to get to this valve, valves visited in path)
        let mut valves_to_check: VecDeque<(&'a str, usize, Vec<&'a str>)> = VecDeque::new();
        let current_valve = self.valves.get(worker_state.valve_id).unwrap();

        for id in &current_valve.next_valves {
            valves_to_check.push_back((
                &id,
                worker_state.minutes_elapsed + 1,
                vec![current_valve.id],
            ));
        }

        while let Some((id, minutes, valves_in_path)) = valves_to_check.pop_front() {
            let valve = self.valves.get(id).unwrap();
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
                // --
                // visit but don't open
                // apply this relatively arbitrary heuristic depending on which strategy
                // this will likely not work for other inputs, but it happens to work
                // for mine, and reduces the run time to something feasible, if still bad
                let v = self.valves.get(id).unwrap();
                let maybe_skip_limit = match global_state.elephant_state {
                    Some(_) => MAYBE_SKIP_LIMIT_WITH_ELEPHANT,
                    None => MAYBE_SKIP_LIMIT_ALONE,
                };
                if v.flow_rate < maybe_skip_limit {
                    next_moves.push(Move::new(id, valves_in_path.clone(), Visit, minutes, 0));
                }
                // open
                next_moves.push(Move::new(
                    id,
                    valves_in_path,
                    Open,
                    minutes + 1,
                    valve.flow_rate * (TIME_LIMIT - minutes - 1),
                ));
            }
        }

        next_moves
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_new_volcano() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let volcano = Volcano::new(&input);
        assert_eq!(volcano.valves.len(), 10);
        assert_eq!(
            volcano.valves.get("AA"),
            Some(&Valve::new("AA", 0, vec!["DD", "II", "BB"]))
        );
        assert_eq!(
            volcano.valves.get("JJ"),
            Some(&Valve::new("JJ", 21, vec!["II"]))
        );
    }
}
