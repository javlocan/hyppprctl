mod workspaces;

use std::process::Command;

use clap::{Parser, Subcommand};
use workspaces::workspaces::*;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "hyppprctl")]
#[command(bin_name = "hyppprctl")]
struct Hyppprctl {
    #[command(subcommand)]
    commands: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Starts workspaces daemon and listens")]
    Workspaces,
    #[command(about = "Runs commands through audioctl")]
    Audio {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    #[command(external_subcommand)]
    Hyprctl(Vec<String>),
}

fn main() {
    let args = Hyppprctl::parse();
    match args.commands {
        Commands::Workspaces => {
            let _ = Wrkspcs::listen();
        }
        Commands::Audio { args } => {
            Command::new("audioctl")
                .args(args)
                .output()
                .expect("Audioctl is not installed");
        }
        Commands::Hyprctl(args) => {
            let cmd = Command::new("hyprctl")
                .args(args)
                .output()
                .expect("Hyprland is not installed??");
            let stdout = String::from_utf8(cmd.stdout).unwrap();
            println!("{}", stdout);
        }
    }
}
