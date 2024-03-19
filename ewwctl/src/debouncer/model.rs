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

pub struct GlobalDebounceServer {
    pub server: GlobalDebouncer,
    pub dbnc_r: Receiver<Action>,
    pub dbnc_t: Sender<Action>,
    pub main_t: Sender<Action>,
}

#[derive(Clone)]
pub struct GlobalDebouncer(pub HashMap<Event, EventDebounceServer>);

#[derive(Clone)]
pub struct EventDebounceServer(pub Arc<Mutex<EventDebounce>>);

pub struct EventDebounce {
    pub sender: Sender<Action>,
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

impl Deref for GlobalDebounceServer {
    type Target = GlobalDebouncer;
    fn deref(&self) -> &Self::Target {
        &self.server
    }
}

impl DerefMut for GlobalDebounceServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.server
    }
}

impl Deref for GlobalDebouncer {
    type Target = HashMap<Event, EventDebounceServer>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GlobalDebouncer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for EventDebounceServer {
    type Target = Arc<Mutex<EventDebounce>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EventDebounceServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
