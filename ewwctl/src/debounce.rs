use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::cli::{Action, Event, Module};

impl Action {
    pub fn is_debounced(&self) -> &bool {
        return &self.debounce;
    }
    pub fn cancels_debounce(&self) -> bool {
        return &self.event == &Event::Hoverlost;
    }
    pub fn without_debounce(mut self) -> Self {
        self.debounce = false;
        return self;
    }
}

impl Debounce {
    pub fn aint_debouncing(&self, event: Event) -> bool {
        return !self.state.contains_key(&event);
    }
}

impl DebounceState {
    pub fn set_debounce(&self, action: Action) {
        let mut debounce = self.lock().unwrap();
        debounce.state.insert(
            action.event.clone(),
            TimedModule {
                module: action.module.clone(),
                time: Some(Instant::now() + Duration::from_millis(1000)),
            },
        );
    }
}

#[derive(Clone)]
pub struct DebounceState(pub Arc<Mutex<Debounce>>);
impl Deref for DebounceState {
    type Target = Arc<Mutex<Debounce>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for DebounceState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
pub struct Debounce {
    pub state: HashMap<Event, TimedModule>,
}
#[derive(Debug)]
pub struct TimedModule {
    pub module: Module,
    pub time: Option<Instant>,
}

impl TimedModule {
    pub fn is_done(&self) -> bool {
        return &self.time.unwrap() <= &Instant::now();
    }
    pub fn is_cancelled(&self) -> bool {
        return self.time.is_none();
    }
}
