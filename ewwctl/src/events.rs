use std::net::UdpSocket;

use crate::cli::Arguments;

const SOCKET_ADDR: &str = "127.0.0.1:9000";

impl Arguments {
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
