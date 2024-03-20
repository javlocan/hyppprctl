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
    debouncer::model::{EventDebounce, GlobalDebouncer, TimedModule},
};

use super::model::{EventDebounceServer, GlobalDebounceServer};

impl GlobalDebounceServer {
    pub fn start_for(&mut self, action: Action) -> () {
        let (sender, receiver) = channel::<Action>();
        let debounce_server = EventDebounce {
            sender: sender.clone(),
            state: Some(TimedModule {
                module: action.module.clone(),
                time: Instant::now() + Duration::from_millis(1000),
            }),
        };
        let debounce_server = EventDebounceServer(Arc::new(Mutex::new(debounce_server)));

        self.insert(action.event.clone(), debounce_server.clone());

        // let d = sender.clone();
        let d = self.dbnc_t.clone();
        let a = action.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            let _ = d.send(a.undebounced());
        });

        let main_t = self.main_t.clone();
        let d = self.dbnc_t.clone();

        thread::spawn(move || {
            while debounce_server.lock().unwrap().state.is_some() {
                let action = receiver.recv().unwrap();

                match action.debounce {
                    true => {
                        // let sender = debounce_server.lock().unwrap().sender.clone();
                        let d = d.clone();
                        let _ = thread::spawn(move || {
                            thread::sleep(Duration::from_millis(1000));
                            let _ = d.send(action.undebounced());
                        });
                    }
                    false => {
                        let state = &debounce_server.lock().unwrap().state;
                        let time = state.as_ref().unwrap();

                        match time.has_passed() {
                            true => {
                                let _ = main_t.lock().unwrap().send(action);
                            }
                            false => {
                                let _ = sender.send(action);
                            }
                        }
                        debounce_server.lock().unwrap().state = None;
                    }
                }
            }
        });
    }

    pub fn handle_action(&mut self, action: Action) -> () {
        let debounce_server = self.get_mut(&action.event.or_associated_event()).unwrap();
        let sender = debounce_server.lock().unwrap().sender.clone();

        match action.cancels_debounce() {
            true => {
                debounce_server.lock().unwrap().state = None;
            }
            false => match action.debounce {
                true => {
                    debounce_server.lock().unwrap().state = Some(TimedModule {
                        module: action.module.clone(),
                        time: Instant::now() + Duration::from_millis(1000),
                    })
                }
                false => {}
            },
        }

        if let Err(err) = sender.send(action) {
            self.remove(&err.0.event);
        };
    }

    pub fn init(
        (dbnc_r, dbnc_t): (Receiver<Action>, Sender<Action>),
        main_t: Sender<Action>,
    ) -> Self {
        let server = GlobalDebouncer(HashMap::new());
        let main_t = Arc::new(Mutex::new(main_t));
        Self {
            server,
            dbnc_r,
            dbnc_t,
            main_t, // dbnc_t: Arc::new(Mutex::new(dbnc_t)),
                    // main_t: Arc::new(Mutex::new(main_t)),
        }
    }
}

impl Action {
    pub fn undebounced(mut self) -> Self {
        self.debounce = false;
        self
    }

    pub fn is_being_debounced(&self, server: &GlobalDebounceServer) -> bool {
        server.contains_key(&self.event.or_associated_event())
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
