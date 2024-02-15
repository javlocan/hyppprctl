mod workspaces;

use std::env;

use audioctl::volume::*;
use workspaces::workspaces::*;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let args: &Vec<String> = &args[1..].to_vec();
    dbg!("{}", args);

    // If there're no arguments
    if args.is_empty() {
        println!("{}", say_bye())
    }
    // Checking arguments
    else {
        if args[1] == "workspaces" {
            Wrkspcs::listen().unwrap()
        } else if args[1] == "volume" {
            if args.len() == 1 {
                Vlm::listen().unwrap();
            } else if args.len() > 1 {
                Vlm::set(args[2..=args.len()].to_vec())
            }
        }
    }
}

fn say_bye() -> &'static str {
    "Bye!"
}
