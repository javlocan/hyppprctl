mod cli;
mod events;

use std::{
    io::Error,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    process::Command,
    sync::{mpsc::sync_channel, Arc, Mutex},
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
            let _ = run_server();
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
            arguments.send_event();
        }
    }
}

fn run_server() -> Result<(), Error> {
    let (tx, rx) = sync_channel::<Action>(0);
    let ty = tx.clone();

    thread::spawn(move || -> Result<(), Error> {
        println!("Hello! Welcome to disgustland");

        let socket = UdpSocket::bind(SOCKET_ADDR)?;
        let mut buf = [0; 4096];
        // This part of the code will handle the props in threads
        loop {
            let (num_bytes, _src_addr) = socket.recv_from(&mut buf)?;
            let msg = std::str::from_utf8(&buf[..num_bytes]).expect("Error converting to UTF-8");
            let action = Action::from_msg(msg);
            println!("{:?}", action);
            let _ = ty.send(action);
        }
    });

    let debounced: Arc<Mutex<Option<Module>>> = Arc::new(Mutex::new(None));

    loop {
        let tz = tx.clone();
        let action = rx.recv().unwrap();
        let arg = &action.module.to_string();

        let mut debounced = debounced.lock().unwrap();

        match action.event {
            Event::Hover => {
                let _ = match action.prop {
                    None => {
                        Command::new("eww").arg("open").arg(arg).output().unwrap();
                    }
                    Some(Prop::Debounce) => match *debounced {
                        None => {
                            *debounced = Some(action.module.clone());
                            thread::spawn(move || {
                                thread::sleep(Duration::from_millis(1300));
                                let _ = tz.send(action);
                            });
                        }
                        Some(Module::Volume) | Some(Module::Wifi) | Some(Module::Brightness) => {
                            if debounced.clone().unwrap() == action.module {
                                *debounced = None;
                                Command::new("eww").arg("open").arg(arg).output().unwrap();
                            } else {
                                *debounced = Some(action.module.clone());
                                let _ = tx.send(action);
                            }
                        }
                    },
                };
            }
            Event::Hoverlost => {}
        }
    }
}
