mod cli;
mod events;

use std::{
    io::Error,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    process::Command,
    sync::{
        mpsc::{channel, sync_channel},
        Arc, Mutex,
    },
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
    let (tx, rx) = channel();
    let (t_dbnc, r_dbnc) = channel();
    // let (tx, rx) = channel();
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
            if !action.debounce {
                let _ = ty.send(action);
            } else {
                let _ = t_dbnc.send(action);
            }
        }
    });
    let debounce: Arc<Mutex<Option<Module>>> = Arc::new(Mutex::new(None));

    // -------------------------------------------------------
    // this is a debouncing thread
    // ------------------------------------------------------

    let tz = tx.clone();
    thread::spawn(move || loop {
        let action = r_dbnc.recv().unwrap();
        match r_dbnc.recv_timeout(Duration::from_millis(300)) {
            Ok(_) => {}
            Err(_) => {
                tz.send(action);
            }
        }
    });

    let hover: Arc<Mutex<Option<Module>>> = Arc::new(Mutex::new(None));

    loop {
        let action = rx.recv().unwrap();

        let arg = &action.module.to_string();

        println!("-------------------------------------------");
        println!("RECEIVED: {:?}", action);
        println!("-------------------------------------------");

        // tengo k ir lockeando donde sea
        let debouncing = Arc::clone(&debounce);

        // MAYBE STATE with hovered and debounced

        let tz = tx.clone();
        match action.event {
            Event::Hover => {
                if !action.debounce {
                    // CAMBIAR ESTO
                    let _ = Command::new("eww").arg("open").arg(arg).output().unwrap();

                    // si viene debounced, no abre nunca ventana !! quitar Command::new
                } else {
                    match *debouncing {
                        None => {
                            // we need to clone it to pass it through the channel
                            let debounced = Arc::clone(&debounce);
                            thread::spawn(move || {
                                println!("debouncing");
                                ////////////////////////////////////////////////
                                ////////////////////////////////////////////////
                                ////////////////////////////////////////////////
                                ////////////////////////////////////////////////
                                // tengo un rx2
                                thread::sleep(Duration::from_millis(2000));
                                let mut debounced = debounced.lock().unwrap();
                                *debounced = Some(action.module.clone());
                                let _ = tz.send(action);
                                // let _ = tz.send(Action {
                                //     module: action.module,
                                //     event: action.event,
                                //     debounce: false,
                                // });
                            });
                        }
                        // viene pidiendo debounce , asi k ya vemos
                        Some(Module::Volume) | Some(Module::Wifi) | Some(Module::Brightness) => {
                            let mut debounced = debounce.lock().unwrap();
                            *debounced = if debounced.clone().unwrap() == action.module {
                                None
                            } else {
                                Some(action.module.clone())
                            };
                            let _ = tz.send(Action {
                                module: action.module,
                                event: action.event,
                                debounce: false,
                            });
                        }
                    }
                };
            }

            Event::Hoverlost => {
                let mut debouncing = debounce.lock().unwrap();
                if debouncing.as_ref().unwrap() == &action.module {
                    *debouncing = None;
                }
                //     let mut hovered = hover.lock().unwrap();
                //     let close = hovered.as_ref().unwrap_or_else(|| &action.module);
                //     *hovered = None;
                //     println!("eww close {}", close);
                //     Command::new("eww")
                //         .arg("close")
                //         .arg(close.to_string())
                //         .output()
                //         .unwrap();
            }
        }
    }
}
