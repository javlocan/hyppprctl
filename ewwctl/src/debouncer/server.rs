use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
};

use crate::cli::Action;

use super::model::{DebounceServer, Debouncer};

impl DebounceServer {
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
