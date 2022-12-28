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

    pub fn ineffective_combination_with(&self, other: &Self) -> bool {
        (self.valve_id == other.valve_id && self.action == other.action)
        || (self.action == Action::Visit && other.action == Action::Visit)
    }
}
