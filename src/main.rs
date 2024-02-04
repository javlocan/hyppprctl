mod hypr;

use crate::hypr::workspaces::*;
use std::env;
use std::process::Command;

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
        }
    }
}

fn say_bye() -> &'static str {
    "Bye!"
}
