mod audio;
mod hypr;

use std::env;

use crate::audio::volume::*;
use crate::hypr::workspaces::*;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let args_amount: usize = args.len() - 1;

    // If there're no arguments
    if args_amount == 0 {
        println!("{}", say_bye())
    }
    // Checking arguments
    else {
        if args[1] == "workspaces" {
            Wrkspcs::listen().unwrap()
        } else if args[1] == "volume" {
            if args_amount == 1 {
                Vlm::listen().unwrap();
            } else if args_amount > 1 {
                Vlm::set(args[2..=args_amount].to_vec())
            }
        }
    }
}

fn say_bye() -> &'static str {
    "Bye!"
}
