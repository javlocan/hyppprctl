use clap::{Parser, Subcommand};

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
    #[command(about = "Starts workspaces daemon and listens")]
    Windows,
    #[command(external_subcommand)]
    Eww(Vec<String>),
}

use std::{io::Error, net::UdpSocket, process::Command};

// const LOCALHOST: &str = "127.0.0.1";
// const PORT: &str = "7878";
const SOCKET_ADDR: &str = "127.0.0.1:9000";

fn main() {
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
    }
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

        print!("{}: {}", src_addr, arguments);

        // Echo the received data back to the client
        socket.send_to(&buf[..num_bytes], src_addr)?;
    }
}
