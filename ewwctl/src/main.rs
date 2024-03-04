mod cli;
mod events;
use std::{
    io::Error,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    process::Command,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
    time::{Duration, Instant},
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
    let (dbnc_t, dbnc_r) = channel();
    let (main_t, main_r) = channel();
    // let (tx, rx) = channel();

    let from_udp_main_t = main_t.clone();
    let from_udp_dbnc_t = dbnc_t.clone();

    thread::spawn(move || -> Result<(), Error> {
        println!("Hello! Welcome to disgustland");

        let socket = UdpSocket::bind(SOCKET_ADDR)?;
        let mut buf = [0; 4096];
        // THIS LOOP RECEIVES BY UDP AND SENDS TO THE PROGRAM BY MPSC
        loop {
            let (num_bytes, _src_addr) = socket.recv_from(&mut buf)?;
            let msg = std::str::from_utf8(&buf[..num_bytes]).expect("Error converting to UTF-8");
            let action = Action::from_msg(msg);
            if !action.debounce {
                let _ = from_udp_main_t.send(action);
            } else {
                let _ = from_udp_dbnc_t.send(action);
            }
        }
    });

    let dbncd: Arc<Mutex<Option<Action>>> = Arc::new(Mutex::new(None));

    let main_t = main_t.clone();
    let first_dbnc_t = dbnc_t.clone();

    let dbncd_action = Arc::clone(&dbncd);

    // ------------------------------------------------------
    // ------------------------ this is the debouncing thread
    // ------------------------------------------------------

    thread::spawn(move || {
        //
        // La primera acción que llega se demora inmediatamente
        // * a la vez registramos el momento actual y el límite

        let mut action_moment = Instant::now();
        let mut deadline = Instant::now() + Duration::from_millis(300);

        let first_action = dbnc_r.recv().unwrap();

        thread::spawn(move || {
            // *hovered_handle.lock().unwrap() = Some(action.module.clone());
            thread::sleep(deadline - action_moment);
            let _ = first_dbnc_t.send(first_action);
        });

        // let hovered = Arc::clone(&hovered);

        loop {
            //
            // Tras la primera acción aquí se bloquea para escucharla
            // * se utiliza el tiempo calculado previamente

            let duration = deadline - action_moment;
            let action = dbnc_r.recv_timeout(duration);

            let dbncd_handle = Arc::clone(&dbncd_action);
            let mut dbncd = dbncd_handle.lock().unwrap();
            // let hovered_handle = Arc::clone(&hovered_handle);
            // let mut hovered = hovered_handle.lock().unwrap();
            action_moment = Instant::now();

            match action {
                Ok(action) => {
                    // Aquí: hemos recibido un evento antes de esperar el tiempo del debounce
                    // Consumimos el evento, cambiamos el estado y reiniciamos

                    // *hovered = Some(action.module);
                    *dbncd = Some(action);
                    deadline = action_moment + Duration::from_millis(300);
                }
                Err(_) => {
                    // Los 300ms han pasado --> CASO A: tenemos evento y es el mismo: open
                    //                          CASO B: tenemos distinto evento: re-debounce

                    // MIERDAAAAAA DE DONDE SACO LA ACCION ORIGINAL?????
                    // ESTO EN REALIDAD ES UN ERROR
                    match dbncd.clone() {
                        None => {}
                        Some(action) => {
                            let _ = main_t.send(action.clone());
                        }
                    }
                }
            }
        }
    });

    // ------------------------------------------------------
    // -------------------------- this is the main event loop
    // ------------------------------------------------------

    let hovered_handle = Arc::clone(&dbncd);

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
                *hovered_handle.lock().unwrap() = None;
            }
        }
    }
}
