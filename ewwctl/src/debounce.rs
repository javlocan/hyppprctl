use std::{collections::HashMap, time::Instant};

use crate::cli::{Action, Event, Module};

impl Action {
    pub fn is_debounced(&self) -> &bool {
        return &self.debounce;
    }
}

impl Debounce {
    pub fn aint_debouncing(&self, event: Event) -> bool {
        return !self.state.contains_key(&event);
    }
}
impl TimedModule {
    pub fn is_done(&self) -> bool {
        return &self.time.1 <= &Instant::now();
    }
}
pub struct Debounce {
    pub state: HashMap<Event, TimedModule>,
}
pub struct TimedModule {
    pub module: Module,
    pub time: (Instant, Instant),
}
