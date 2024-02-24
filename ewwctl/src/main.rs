use std::{io::Error, net::UdpSocket, process::Command, sync::Mutex, thread, time};

use clap::{Args, Parser, Subcommand, ValueEnum};
//
// redo with ipv4 shit
// const LOCALHOST: &str = "127.0.0.1";
// const PORT: &str = "7878";
const SOCKET_ADDR: &str = "127.0.0.1:9000";

#[derive(Parser, Debug)]
#[command(name = "ewwctl")]
#[command(bin_name = "ewwctl")]
#[command(arg_required_else_help = true)]
#[command(subcommand_help_heading = "Daemon")]
#[command(subcommand_value_name = "start - help")]
#[command(subcommand_negates_reqs = true)]
#[command(next_help_heading = "Actions")]
struct Ewwctl {
    #[command(subcommand)]
    commands: Option<Commands>,
    #[command(flatten)]
    args: Option<Arguments>,
}
#[derive(Args, Debug)]
struct Arguments {
    /// Module that emmits the event
    /// *
    #[arg(verbatim_doc_comment)]
    #[arg(short, long)]
    module: Module,
    /// e - Emmits events for the socket to process
    /// Event types:
    /// Hover       <module>
    /// Hoverlost   <module>
    /// For more information: ewwctl -e -h
    /// *
    #[arg(verbatim_doc_comment)]
    #[arg(short, long)]
    event: Event,
}
#[derive(Debug, Subcommand)]
enum Commands {
    /// Starts the socket and listens to the output.
    #[command(allow_missing_positional = true)]
    Start,
    #[command(external_subcommand)]
    Eww(Vec<String>),
}
#[derive(Debug, Clone, ValueEnum)]
enum Event {
    Hover,
    Hoverlost,
}

#[derive(Debug, Clone, ValueEnum)]
enum Module {
    Volume,
    Brightness,
    Wifi,
}
struct State {
    module: Module,
}

fn main() {
    let args = Ewwctl::parse();

    // match args.commands {
    //     Commands::Start => run_socket().expect("Socket has failed"),
    //     // Commands::Event(args) => match args.command {
    //     //     Some(EventCommands::Hoverlost { module }) | Some(EventCommands::Hover { module }) => {
    //     //         send_event(module, None);
    //     //     }
    //     //     None => {}
    //     // },
    //     Commands::Eww(args) => {
    //         let cmd = Command::new("eww")
    //             .args(args)
    //             .output()
    //             .expect("Eww is not installed??");
    //         let stdout = String::from_utf8(cmd.stdout).unwrap();
    //         println!("{}", stdout)
    //     }
    // }
}

fn send_event(module: Module, debounce: Option<u64>) {
    let msg = module.to_possible_value().unwrap();
    let msg = msg.get_name();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("s");

    let msg = format!("module:{}", msg);
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
