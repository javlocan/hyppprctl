mod cli;
mod events;

use std::{
    io::Error,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    process::Command,
    sync::Mutex,
    thread,
    time::Duration,
};

use crate::cli::*;
use clap::Parser;

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 9000;
const SOCKET_ADDR: SocketAddrV4 = SocketAddrV4::new(LOCALHOST, PORT);
// const SOCKET_ADDR: &str = "127.0.0.1:9000";

fn main() {
    let input = Ewwctl::parse();

    std::env::set_var("RUST_BACKTRACE", "1");
    // First we'll handle the possible commands.
    match input.commands {
        Some(Commands::Start) => {
            println!("Ewwctl socket is running on {}!", SOCKET_ADDR);
            let _ = run_socket();
        }
        Some(Commands::Eww(args)) => {
            let cmd = Command::new("eww")
                .args(args)
                .output()
                .expect("Eww is not installed?? Bad new then!");
            println!("{}", String::from_utf8(cmd.stdout).unwrap())
        }
        None => {
            // If no command is passed, we handle the arguments
            let arguments = input.args.unwrap();
            arguments.send_event(Some(300));
        }
    }
}

fn run_socket() -> Result<(), Error> {
    // Here we initializate the state.
    let state: Mutex<Option<Module>> = Mutex::new(None);
    let debounced_hover: Mutex<Option<Module>> = Mutex::new(None);

    // Bind the UDP socket to local address 127.0.0.1:9000
    let socket = UdpSocket::bind(SOCKET_ADDR)?;
    // Buffer to store incoming data
    let mut buf = [0; 4096];

    println!("Hello! Welcome to disgustland");

    loop {
        // Receive arguments from UDP client
        let (num_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let msg = std::str::from_utf8(&buf[..num_bytes]).expect("Error converting to UTF-8");
        let action = Arguments::from_msg(msg);

        match action.event {
            Event::Hover => {
                let _ = thread::spawn(move || {
                    if debounced_hover.lock().unwrap().is_some() {
                        thread::sleep(Duration::from_millis(300));
                        *state.lock().unwrap() = debounced_hover.lock().unwrap().take();
                    } else {
                    }
                    let soc = UdpSocket::bind("0.0.0.0:0").expect("s");
                    // let _ = soc.send_to(msg.as_bytes(), SOCKET_ADDR);
                });
            }

            Event::Hoverlost => {}
        }

        println!("Event: {:?}", msg);

        // Echo the received data back to the client
        socket.send_to(&buf[..num_bytes], src_addr)?;
    }
}
