use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use crate::{
    cli::{Action, Event},
    debouncer::model::{EventDebounceServer, EventDebouncer, TimedModule},
};

use super::model::{DebounceServer, Debouncer};

impl Action {
    pub fn is_being_debounced(&self, server: &DebounceServer) -> bool {
        server.contains_key(&self.event.or_associated_event())
    }
}

impl Event {
    pub fn or_associated_event(&self) -> Self {
        match self {
            Self::Hoverlost => Self::Hover,
            other => other.clone(),
        }
    }
}

impl DebounceServer {
    pub fn start_for(&mut self, action: Action) -> () {
        let (event_t, event_r) = channel::<Action>();
        let debounce_server = EventDebounceServer {
            channel: (event_t, event_r),
            state: Some(TimedModule {
                module: action.module,
                time: Instant::now() + Duration::from_millis(1000),
            }),
        };
        let debounce_server = Arc::new(Mutex::new(debounce_server));
        let event_debouncer = EventDebouncer(debounce_server);
        self.insert(action.event, event_debouncer);
        // loop {}
        println!("handleiiiiiiing");
    }

    pub fn handle_action(&self, action: &Action) -> () {
        println!("handleiiiiiiing");
    }

    pub fn init(
        (dbnc_r, dbnc_t): (Receiver<Action>, Sender<Action>),
        main_t: Sender<Action>,
    ) -> Self {
        DebounceServer {
            server: Debouncer(HashMap::new()),
            dbnc_r,
            dbnc_t,
            main_t,
        }
    }
}
