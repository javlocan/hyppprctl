use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ewwctl")]
#[command(bin_name = "ewwctl")]
struct Ewwctl {
    #[command(subcommand)]
    commands: Commands,
    // #[arg(short = 'm', long = "monitors", id = "monitors", required = false)]
    // monitors: u8,
}
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Starts eww windows daemon and listens")]
    Windows,
    #[command(about = "Starts eww windows daemon and listens")]
    Hover(HoverArgs),
    #[command(external_subcommand)]
    Eww(Vec<String>),
}

#[derive(Args, Debug)]
struct HoverArgs {
    #[arg(short = 'm', name = "module")]
    module: String,
}
use std::{io::Error, net::UdpSocket, process::Command, sync::Mutex};

// const LOCALHOST: &str = "127.0.0.1";
// const PORT: &str = "7878";
const SOCKET_ADDR: &str = "127.0.0.1:9000";

fn main() {
    #[allow(non_snake_case)]
    let mut HOVER_STATE: Mutex<Vec<String>> = Mutex::new(vec![]);

    let args = Ewwctl::parse();

    match args.commands {
        Commands::Eww(args) => {
            let cmd = Command::new("eww")
                .args(args)
                .output()
                .expect("Eww is not installed??");
            let stdout = String::from_utf8(cmd.stdout).unwrap();
            println!("{}", stdout)
        }
        Commands::Windows => run_socket().expect("Socket has failed"),
        Commands::Hover(args) => {
            let hovered = HOVER_STATE.lock().expect("Cannot open hover_state Mutex");

            if hovered.contains(&args.module) {
                HOVER_STATE
                    .get_mut()
                    .expect("Cannot obtain mutable reference to hover_state Mutex")
                    .iter()
                    .filter(predicate);
            }
            // hover_state.push(args.module);
            println!("{:?}", hover_state);

            let socket = UdpSocket::bind("0.0.0.0:0").expect("s");
            let _ = socket.send_to(args.module.as_bytes(), SOCKET_ADDR);
        }
    }
}

fn send_debounced_event(arg: HoverArgs) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("s");
    let _ = socket.send_to(arg.module.as_bytes(), SOCKET_ADDR);
}

fn run_socket() -> Result<(), Error> {
    // Bind the UDP socket to local address 127.0.0.1:9000
    let socket = UdpSocket::bind(SOCKET_ADDR)?;

    println!("Hello! Welcome to disgustland");
    // Buffer to store incoming data
    let mut buf = [0; 4096];

    loop {
        // Receive arguments from UDP client
        let (num_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let arguments = std::str::from_utf8(&buf[..num_bytes]).expect("Error converting to UTF-8");

        if arguments.contains("mierda") {
            println!("{}: {}", src_addr, arguments);
        } else {
            println!("NONAY MIERDA")
        }

        // Echo the received data back to the client
        socket.send_to(&buf[..num_bytes], src_addr)?;
    }
}
