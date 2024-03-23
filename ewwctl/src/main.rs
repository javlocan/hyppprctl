mod cli;
mod debouncer;

use std::{
    io::Error,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    process::Command,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

use crate::cli::*;
use clap::Parser;
use debouncer::model::GlobalDebounceServer;

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 9000;
const SOCKET_ADDR: SocketAddrV4 = SocketAddrV4::new(LOCALHOST, PORT);

fn main() {
    let input = Ewwctl::parse();

    std::env::set_var("RUST_BACKTRACE", "1");
    // First we'll handle the possible commands.
    match input.commands {
        Some(Commands::Start) => {
            println!("Ewwctl socket is running on {}!", SOCKET_ADDR);
            let _ = run_server();
        }
        Some(Commands::Eww(args)) => {
            let cmd = Command::new("eww")
                .args(args)
                .output()
                .expect("Eww is not installed?? Bad news then!");
            println!("{}", String::from_utf8(cmd.stdout).unwrap())
        }
        None => {
            // If no command is passed, we handle the arguments
            let arguments = input.args.unwrap();
            arguments.send_event();
        }
    }
}

fn run_server() -> Result<(), Error> {
    let (dbnc_t, dbnc_r) = channel();
    let (main_t, main_r) = channel();
    // let (tx, rx) = channel();

    let from_udp_main_t = main_t.clone();
    let from_udp_dbnc_t = dbnc_t.clone();

    // ------------------------------------------------------
    // ----------------- this thread translates str to action
    // ------------------------------------------------------

    thread::spawn(move || -> Result<(), Error> {
        // println!("Hello! Welcome to disgustland");

        let socket = UdpSocket::bind(SOCKET_ADDR)?;
        let mut buf = [0; 4096];

        loop {
            let (num_bytes, _src_addr) = socket.recv_from(&mut buf)?;
            let msg = std::str::from_utf8(&buf[..num_bytes]).expect("Error converting to UTF-8");
            let action = Action::from_msg(msg);

            if *action.is_debounced() || action.cancels_debounce() {
                let _ = from_udp_dbnc_t.send(action);
            } else {
                let _ = from_udp_main_t.send(action);
            }
        }
    });

    // ------------------------------------------------------
    // ------------------------ this is the debouncing thread
    // ------------------------------------------------------

    thread::spawn(move || {
        let mut server = GlobalDebounceServer::init(dbnc_r, dbnc_t.clone());
        let mut i = 0;

        loop {
            println!("-----------------------------------");
            println!("[WAITING FOR ACTION TO DEBOUNCE]");
            let action = server.dbnc_r.recv().unwrap();

            i += 1;

            println!(
                "[{}] {} {} on {} / {}",
                i,
                match action.is_being_debounced(&server.server) {
                    true => "Handling event",
                    false => "Starting server for",
                },
                action.event.or_associated_event().to_string(),
                action.module.to_string(),
                match action.is_debounced() {
                    true => "debouncing",
                    false => "processing",
                },
            );
            println!("-----------------------------------");

            match action.is_being_debounced(&server.server) {
                true => server.handle_action(action),
                false => server.start_for(action, main_t.clone()),
            };
        }
    });

    // ------------------------------------------------------
    // -------------------------- this is the main event loop
    // ------------------------------------------------------

    // let dbnc = Arc::clone(&debounce);
    let windows: Arc<Mutex<Option<Module>>> = Arc::new(Mutex::new(None));

    loop {
        let action = main_r.recv().unwrap();
        let action_module = &action.module.to_string();

        match action.event {
            Event::Hover => {
                *windows.lock().unwrap() = Some(action.module);
                Command::new("eww")
                    .arg("open")
                    .arg(action_module)
                    .output()
                    .unwrap();
            }
            Event::Hoverlost => {
                println!("hoverlost: {}", action.module.to_string());
                *windows.lock().unwrap() = None;
                Command::new("eww")
                    .arg("close")
                    .arg(action_module)
                    .output()
                    .unwrap();
            }
        }
    }
}
