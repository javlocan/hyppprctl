mod cli;
mod events;

use std::{io::Error, net::UdpSocket, process::Command, sync::Mutex};

use crate::cli::*;
use clap::Parser;

// redo with ipv4 shit
// const LOCALHOST: &str = "127.0.0.1";
// const PORT: &str = "7878";
const SOCKET_ADDR: &str = "127.0.0.1:9000";

fn main() {
    let input = Ewwctl::parse();

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
    // Bind the UDP socket to local address 127.0.0.1:9000
    let socket = UdpSocket::bind(SOCKET_ADDR)?;
    println!("Hello! Welcome to disgustland");

    // Here we initializate the state.
    #[allow(non_snake_case)]
    let STATE: Mutex<Option<Module>> = Mutex::new(None);

    let debounced_hover: Mutex<Option<Module>> = Mutex::new(None);
    // Buffer to store incoming data
    let mut buf = [0; 4096];

    loop {
        // Receive arguments from UDP client
        let (num_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let msg = std::str::from_utf8(&buf[..num_bytes]).expect("Error converting to UTF-8");
        let msg = Arguments::from_msg(msg);

        println!("{:?}", msg);

        // Echo the received data back to the client
        socket.send_to(&buf[..num_bytes], src_addr)?;
    }
}
