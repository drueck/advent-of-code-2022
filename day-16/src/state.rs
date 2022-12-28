use crate::moves::{Action, Move};
use std::collections::HashSet;

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
        if m.action != Action::Wait {
            self.human_state = WorkerState::new(m.valve_id, m.minutes_elapsed);
            self.apply_move(&m);
        }
    }

    pub fn apply_elephant_move(&mut self, m: &Move<'a>) {
        if m.action != Action::Wait {
            self.elephant_state = Some(WorkerState::new(m.valve_id, m.minutes_elapsed));
            self.apply_move(&m);
        }
    }

    fn apply_move(&mut self, m: &Move<'a>) {
        match m.action {
            Action::Visit => {
                self.visited_valves.insert(m.valve_id);
                self.add_visited_valves(&m.valves_in_path);
            }
            Action::Open => {
                self.open_valves.insert(m.valve_id);
                self.total_pressure += m.pressure_released;
                self.add_visited_valves(&m.valves_in_path);
            }
            Action::Wait => {}
        }
    }

    fn add_visited_valves(&mut self, valves: &Vec<&'a str>) {
        for v in valves {
            if !self.open_valves.contains(v) {
                self.visited_valves.insert(v);
            }
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
