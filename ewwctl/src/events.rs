use std::{net::UdpSocket, str::from_utf8};

use crate::cli::{Action, Event, Module, Prop};
use clap::ValueEnum;

const SOCKET_ADDR: &str = "127.0.0.1:9000";

impl Action {
    pub fn from_msg(msg: &str) -> Action {
        let &collon = &msg.rfind(':').unwrap();
        let &equal = &msg.rfind('=').unwrap_or_else(move || msg.len());

        let module = &msg[..collon];
        let module = Module::from_str(module, true).unwrap();
        let event = &msg[collon + 1..equal];
        let event = Event::from_str(event, true).unwrap();
        let prop = &msg[equal + 1..];
        let prop = println!("{}", prop);
        let prop = Some(Prop::from_str(prop, true).unwrap()).expect("No proppin is fucking me up");

        Action {
            module,
            event,
            prop: Some(prop),
        }
    }
    // pub fn open_module_window() {}

    // pub fn send_event(&self, debounce: Option<u64>) {}
    pub fn send_event(&self) {
        let prop = match &self.prop {
            Some(..) => &self.prop.unwrap(),
            None => None,
        };
        let msg = format!("{:#?}:{:#?}={:#?}", &self.module, &self.event, &self.prop);

        println!("{}", msg);

        let socket = UdpSocket::bind("0.0.0.0:0").expect("s");
        let _ = socket.send_to(msg.as_bytes(), SOCKET_ADDR);
    }
}
