mod cli;
mod debounce;
mod events;

use std::{
    collections::HashMap,
    io::Error,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    process::Command,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::{cli::*, debounce::TimedModule};
use clap::Parser;
use debounce::Debounce;

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

    thread::spawn(move || -> Result<(), Error> {
        println!("Hello! Welcome to disgustland");

        let socket = UdpSocket::bind(SOCKET_ADDR)?;
        let mut buf = [0; 4096];

        loop {
            let (num_bytes, _src_addr) = socket.recv_from(&mut buf)?;
            let msg = std::str::from_utf8(&buf[..num_bytes]).expect("Error converting to UTF-8");
            let action = Action::from_msg(msg);

            println!(
                "[UDP] sending: {} {}",
                action.module.to_string(),
                action.event.to_string()
            );
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

    let debounce: Arc<Mutex<Debounce>> = Arc::new(Mutex::new(Debounce {
        state: HashMap::new(),
    }));
    let dbnc = debounce.clone();

    let main_t = main_t.clone();
    let dbnc_t = dbnc_t.clone();

    thread::spawn(move || {
        let mut i = 0;

        loop {
            i += 1;
            println!("[WAITING FOR ACTION TO DEBOUNCE]");
            println!("[DBNC STATE] {:?}", &debounce.lock().unwrap().state);
            let action = dbnc_r.recv().unwrap();
            let dbnc = Arc::clone(&dbnc);

            println!("loop:{} successfuly received action {:?}", i, action);
            // ¿La acción que viene debouncea o cancela debounce?
            // Cancela
            //
            if action.cancels_debounce() {
                // ----------------------
                // ------- CANCEL -------
                // ----------------------
                let mut dbnc = dbnc.lock().unwrap();
                let event = Event::Hover;
                if dbnc.state.contains_key(&event) {
                    dbnc.state.get_mut(&event).unwrap().time = None;
                }
                let _ = main_t.send(action);
            } else if action.debounce {
                // ----------------------
                // ----- DEBOUNCING -----
                // ----------------------
                match dbnc.lock().unwrap().state.get(&action.event) {
                    None => {
                        let mut dbnc = dbnc.lock().unwrap();
                        dbnc.state.insert(
                            action.event.clone(),
                            TimedModule {
                                module: action.module.clone(),
                                time: Some(Instant::now() + Duration::from_millis(1000)),
                            },
                        );
                        let dbnc_t = dbnc_t.clone();
                        thread::spawn(move || {
                            thread::sleep(Duration::from_millis(1000));
                            let _ = dbnc_t.send(action);
                        });
                    }
                    Some(TimedModule { module, time }) => match time {
                        None => {
                            // SI NO HAY TIEMPO EN EL TIMED MODULE -> Cancelaaaao
                        }
                        Some(end) => {
                            // Hay un time en el timedmodule => camino normal
                            match end.checked_duration_since(Instant::now()) {
                                None => {
                                    // EVENTO TERMINAO
                                    dbnc.lock().unwrap().state.remove(&action.event);
                                    let _ = dbnc_t.send(action.without_debounce());
                                }
                                Some(_) => {
                                    dbnc.lock()
                                        .unwrap()
                                        .state
                                        .get_mut(&action.event)
                                        .unwrap()
                                        .time = Some(Instant::now() + Duration::from_millis(1000));
                                    let dbnc_t = dbnc_t.clone();
                                    thread::spawn(move || {
                                        thread::sleep(Duration::from_millis(1000));
                                        let _ = dbnc_t.send(action);
                                    });
                                }
                            }
                        }
                    },
                }
            } else {
                // if !action.debounce
                match dbnc.lock().unwrap().state.get(&action.event) {
                    // AQUI NO TENGO NI IDEA AUN
                    None => {
                        let _ = main_t.send(action);
                    }
                    Some(timedmodule) => {}
                }
            }
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
