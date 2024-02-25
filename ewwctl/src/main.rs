mod cli;
use std::{io::Error, net::UdpSocket, process::Command, sync::Mutex, thread, time};

use clap::Parser;

use crate::cli::*;

// redo with ipv4 shit
// const LOCALHOST: &str = "127.0.0.1";
// const PORT: &str = "7878";
const SOCKET_ADDR: &str = "127.0.0.1:9000";

struct State {
    module: Module,
}

fn main() {
    let input = Ewwctl::parse();

    // Parse input
    match input.commands {
        Some(Commands::Start) => {
            let _ = "as";
        }
        Some(Commands::Eww(args)) => {
            let cmd = Command::new("eww")
                .args(args)
                .output()
                .expect("Eww is not installed?? Bad new then!");
            println!("{}", String::from_utf8(cmd.stdout).unwrap())
        }
        None => {}
    }
    match input.args {
        Some(arg) => {}
        None => {}
    }
}

fn send_event(module: Module, debounce: Option<u64>) {
    // let msg = match module {
    //     Module::Wifi => {}
    // };
    let msg = module;

    let socket = UdpSocket::bind("0.0.0.0:0").expect("s");

    let msg = format!("module:{:?}", msg);
    println!("{}", msg);
    let _ = socket.send_to(msg.as_bytes(), SOCKET_ADDR);

    match debounce {
        Some(ms) => {
            thread::sleep(time::Duration::from_millis(ms));
            let _ = socket.send_to(msg.as_bytes(), SOCKET_ADDR);
        }
        None => {}
    }
}

fn run_socket() -> Result<(), Error> {
    // Bind the UDP socket to local address 127.0.0.1:9000
    let socket = UdpSocket::bind(SOCKET_ADDR)?;
    println!("Hello! Welcome to disgustland");

    // Here we initializate the state.
    #[allow(non_snake_case)]
    let STATE: Mutex<Option<Module>> = Mutex::new(None);

    // Buffer to store incoming data
    let mut buf = [0; 4096];

    loop {
        // Receive arguments from UDP client
        let (num_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let msg = std::str::from_utf8(&buf[..num_bytes]).expect("Error converting to UTF-8");

        println!("inside loop: {}", msg);
        if msg.contains("volume") {
            let state = STATE.lock().unwrap();

            println!("{:?}", state);
            // println!("{}: {}", src_addr, arguments);
        } else {
            println!("NONAY MIERDA")
        }

        // Echo the received data back to the client
        socket.send_to(&buf[..num_bytes], src_addr)?;
    }
}
