use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use crate::{
    cli::{Action, Event},
    debouncer::model::{GlobalDebouncer, TimedModule},
};

use super::model::{EventDebounceServer, GlobalDebounceServer};

impl GlobalDebounceServer {
    pub fn start_for(&mut self, action: Action, main_t: Sender<Action>) -> () {
        let (sender, receiver) = channel::<Action>();
        let debounce_server = EventDebounceServer {
            sender: sender.clone(),
            state: Some(TimedModule {
                module: action.module.clone(),
                time: Instant::now() + Duration::from_millis(1000),
            }),
        };

        self.server
            .lock()
            .unwrap()
            .insert(action.event.clone(), debounce_server);

        // let d = sender.clone();
        let d = self.dbnc_t.clone();
        let a = action.clone();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            let _ = d.send(a.undebounced());
        });

        let d = self.dbnc_t.clone();
        let server = self.server.clone();

        thread::spawn(move || {
            while server.lock().unwrap().contains_key(&action.event) {
                let action = receiver.recv().unwrap();

                match action.cancels_debounce() {
                    true => {
                        server
                            .lock()
                            .unwrap()
                            .remove(&action.event.or_associated_event());
                    }
                    false => match action.debounce {
                        true => {
                            let d = d.clone();
                            let _ = thread::spawn(move || {
                                thread::sleep(Duration::from_millis(1000));
                                let _ = d.send(action.undebounced());
                            });
                        }
                        false => {
                            let _ = main_t.send(action);
                        }
                    },
                }
            }
            // esto aqui no va
        });
    }

    pub fn handle_action(&self, action: Action) -> () {
        let sender = self
            .server
            .lock()
            .unwrap()
            .get_mut(&action.event.or_associated_event())
            .unwrap()
            .sender
            .clone();
        println!("get sender for {}", action.event);

        if let Err(err) = sender.send(action) {
            println!("EEEERRROOOOOOOOOOOOORRR")
            // self.server.remove(&err.0.event);
        };
    }

    pub fn init(dbnc_r: Receiver<Action>, dbnc_t: Sender<Action>) -> Self {
        let server = GlobalDebouncer(Arc::new(Mutex::new(HashMap::new())));
        Self {
            server,
            dbnc_r,
            dbnc_t,
        }
    }
}

impl Action {
    pub fn undebounced(mut self) -> Self {
        self.debounce = false;
        self
    }

    pub fn is_being_debounced(&self, server: &GlobalDebouncer) -> bool {
        server
            .lock()
            .unwrap()
            .contains_key(&self.event.or_associated_event())
    }

    pub fn is_debounced(&self) -> &bool {
        return &self.debounce;
    }
    pub fn cancels_debounce(&self) -> bool {
        return &self.event == &Event::Hoverlost;
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
