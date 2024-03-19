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

        self.insert(action.event.clone(), debounce_server);

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            let _ = sender.send(action.undebounced());
        });

        thread::spawn(move || loop {
            let action = receiver.recv().unwrap();
        });
    }

    pub fn handle_action(&self, action: &Action) -> () {
        let debounce_server = self.get(&action.event.or_associated_event()).unwrap();

        // ¿La acción que viene debouncea o cancela debounce?
        // Cancela
        //
        // if action.cancels_debounce() {
        //     // ----------------------
        //     // ------- CANCEL -------
        //     // ----------------------
        //     let mut dbnc = dbnc.lock().unwrap();
        //     let event = Event::Hover;
        //     if dbnc.state.contains_key(&event) {
        //         dbnc.state.get_mut(&event).unwrap().time = None;
        //     }
        //     let _ = main_t.send(action);
        // } else if action.debounce {
        //     // ----------------------
        //     // ----- DEBOUNCING -----
        //     // ----------------------
        //     match dbnc.lock().unwrap().state.get(&action.event) {
        //         None => {
        //             dbnc.set_debounce(action.clone());
        //             let dbnc_t = dbnc_t.clone();
        //             thread::spawn(move || {
        //                 thread::sleep(Duration::from_millis(1000));
        //                 let _ = dbnc_t.send(action);
        //             });
        //         }
        //         Some(TimedModule {
        //             time: Some(end),
        //             module,
        //         }) => {
        //             // Hay un time en el timedmodule => camino normal
        //             match end.checked_duration_since(Instant::now()) {
        //                 None => {
        //                     // EVENTO TERMINAO
        //                     dbnc.lock().unwrap().state.remove(&action.event);
        //                     let _ = dbnc_t.send(action.without_debounce());
        //                 }
        //                 Some(_) => {
        //                     dbnc.lock()
        //                         .unwrap()
        //                         .state
        //                         .get_mut(&action.event)
        //                         .unwrap()
        //                         .time = Some(Instant::now() + Duration::from_millis(1000));
        //                     let dbnc_t = dbnc_t.clone();
        //                     thread::spawn(move || {
        //                         thread::sleep(Duration::from_millis(1000));
        //                         let _ = dbnc_t.send(action);
        //                     });
        //                 }
        //             }
        //         }
        //         Some(TimedModule { time: None, module }) => {}
        //     }
        // } else {
        //     // if !action.debounce
        //     match dbnc.lock().unwrap().state.get(&action.event) {
        //         // AQUI NO TENGO NI IDEA AUN
        //         None => {
        //             let _ = main_t.send(action);
        //         }
        //         Some(timedmodule) => {}
        //     }
        // }
    }

    pub fn init(
        (dbnc_r, dbnc_t): (Receiver<Action>, Sender<Action>),
        main_t: Sender<Action>,
    ) -> Self {
        let server = GlobalDebouncer(HashMap::new());
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
}

impl Event {
    pub fn or_associated_event(&self) -> Self {
        match self {
            Self::Hoverlost => Self::Hover,
            other => other.clone(),
        }
    }
}
