use std::net::UdpSocket;

use crate::cli::{Arguments, Event, Module};
use clap::ValueEnum;

const SOCKET_ADDR: &str = "127.0.0.1:9000";

impl Arguments {
    pub fn from_msg(msg: &str) -> Arguments {
        let &collon = &msg.rfind(':').unwrap();
        let &equal = &msg.rfind('=').unwrap_or_else(move || msg.len());

        let module = &msg[..collon];
        let module = Module::from_str(module, true).unwrap();
        let event = &msg[collon + 1..equal];
        let event = Event::from_str(event, true).unwrap();

        Arguments { module, event }
    }
    pub fn open_module_window() {}

    pub fn send_event(&self, debounce: Option<u64>) {
        let mut msg = format!("{:#?}:{:#?}", &self.module, &self.event);

        if debounce.is_some() {
            msg = format!("{}=debounce", msg);
        }

        println!("{}", msg);

        let socket = UdpSocket::bind("0.0.0.0:0").expect("s");
        let _ = socket.send_to(msg.as_bytes(), SOCKET_ADDR);
    }
}
