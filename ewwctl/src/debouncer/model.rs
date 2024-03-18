use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    time::Instant,
};

use crate::cli::{Action, Event, Module};

pub struct DebounceServer {
    pub server: Debouncer,
    pub dbnc_r: Receiver<Action>,
    pub dbnc_t: Sender<Action>,
    pub main_t: Sender<Action>,
}

#[derive(Clone)]
pub struct Debouncer(pub HashMap<Event, EventDebouncer>);

#[derive(Clone)]
pub struct EventDebouncer(pub Arc<Mutex<EventDebounceServer>>);

pub struct EventDebounceServer {
    pub channel: (Sender<Action>, Receiver<Action>),
    pub state: Option<TimedModule>,
}

#[derive(Debug)]
pub struct TimedModule {
    pub module: Module,
    pub time: Instant,
}

// ------------------------------------------------
// ----------------- Newtype Pattern --------------
// ------------------------------------------------

impl Deref for DebounceServer {
    type Target = Debouncer;
    fn deref(&self) -> &Self::Target {
        &self.server
    }
}

impl DerefMut for DebounceServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.server
    }
}

impl Deref for Debouncer {
    type Target = HashMap<Event, EventDebouncer>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Debouncer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for EventDebouncer {
    type Target = Arc<Mutex<EventDebounceServer>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EventDebouncer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
