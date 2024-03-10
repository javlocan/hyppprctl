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
            if *action.is_debounced() {
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
    let first_dbnc_t = dbnc_t.clone();

    thread::spawn(move || {
        //
        // La primera acción que llega se demora inmediatamente
        // * a la vez registramos el momento actual y el límite

        let mut action_moment: Instant;
        let mut deadline = Instant::now() + Duration::from_millis(1000);
        let mut i = 0;

        loop {
            i += 1;
            println!("loop:{}", i);

            let action = dbnc_r.recv().unwrap();
            println!("loop:{} successfuly received action {:?}", i, action);

            // -------------- this thread applies the delay and sends
            let dbnc = dbnc.clone();
            // let mut dbnc = dbnc.lock().unwrap();

            if dbnc.lock().unwrap().aint_debouncing(action.event.clone()) {
                let dbnc = dbnc.clone();
                let first_dbnc_t = first_dbnc_t.clone();
                thread::spawn(move || {
                    let mut dbnc = dbnc.lock().unwrap();
                    dbnc.state.insert(
                        action.event.clone(),
                        TimedModule {
                            module: action.module.clone(),
                            time: (Instant::now(), Instant::now() + Duration::from_millis(1000)),
                        },
                    );

                    drop(dbnc);
                    thread::sleep(Duration::from_millis(1000));

                    let _ = first_dbnc_t.send(action);
                });
            }

            'inner: loop {
                //
                // Tras la primera acción aquí se bloquea para escucharla

                let action = dbnc_r.recv();
                let hoveraction = action.unwrap();

                let dbnc = Arc::clone(&dbnc);
                let mut dbnc = dbnc.lock().unwrap();

                if dbnc.aint_debouncing(hoveraction.event.clone()) {
                    // CASE:
                    dbnc.state.insert(
                        hoveraction.event.clone(),
                        TimedModule {
                            module: hoveraction.module.clone(),
                            time: (Instant::now(), Instant::now() + Duration::from_millis(1000)),
                        },
                    );
                    break 'inner;
                } else {
                    // CASE is_debouncing:
                    //
                    let debounced_module = dbnc.state.get(&hoveraction.event).unwrap();
                    if debounced_module.is_done() {
                        let _ = main_t.send(hoveraction);
                        // drop(dbnc);
                        break 'inner;
                    } else {
                        let dbnc_t = first_dbnc_t.clone();
                        let duration = debounced_module.time.1 - debounced_module.time.0;

                        thread::spawn(move || {
                            thread::sleep(duration);
                            let _ = dbnc_t.send(hoveraction);
                        });
                    };
                }

                // match action {
                //     Ok(action) => {
                //         // Aquí: hemos recibido un evento antes de esperar el tiempo del debounce
                //         // Consumimos el evento, cambiamos el estado y reiniciamos
                //
                //         action_moment = Instant::now();
                //         deadline = action_moment + Duration::from_millis(1000);
                //
                //         let dbnc = dbnc.state.get_mut(&action.event).unwrap();
                //         *dbnc = TimedModule {
                //             module: action.module,
                //             time: (Instant::now(), Instant::now() + Duration::from_millis(1000)),
                //         };
                //
                //         println!("[while debouncing] hover: {}", action.module.to_string());
                //     }
                //
                //     Err(_) => {
                //         // Los 1000ms han pasado --> CASO A: tenemos evento y es el mismo: open
                //         //                          CASO B: tenemos distinto evento: re-debounce
                //
                //         // let mut dbncd = dbncd.clone();
                //         let _answer = dbnc_r.recv();
                //         let dbncd = dbncd.clone();
                //         match dbncd {
                //             None => {}
                //             Some(action) => {
                //                 println!("opening: {}", action.module.to_string());
                //                 let _ = main_t.send(action.clone());
                //                 // dbncd = None;
                //                 continue 'outer;
                //             }
                //         }
                //     }
                // }
            }
        }
    });

    // ------------------------------------------------------
    // -------------------------- this is the main event loop
    // ------------------------------------------------------

    let dbnc = Arc::clone(&debounce);

    loop {
        let action = main_r.recv().unwrap();
        let action_module = &action.module.to_string();

        match action.event {
            Event::Hover => {
                Command::new("eww")
                    .arg("open")
                    .arg(action_module)
                    .output()
                    .unwrap();
            }
            Event::Hoverlost => {
                println!("hoverlost: {}", action.module.to_string());
                let mut dbnc = dbnc.lock().unwrap();
                dbnc.state.remove_entry(&Event::Hover);
                Command::new("eww")
                    .arg("close")
                    .arg(action_module)
                    .output()
                    .unwrap();
            }
        }
    }
}
