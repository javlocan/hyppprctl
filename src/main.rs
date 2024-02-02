mod hypr;

use crate::hypr::*;
use hyprland::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let args_amount: usize = args.len() - 1;

    // If there're no arguments
    if args_amount == 0 {
        println!("{}", say_bye())
    }
    // Checking arguments
    else {
        if args[1] == "workspaces" {
            let wrkspcs = workspaces::get();
            println!("{:?}", wrkspcs.workspaces);
        }
    }
    Ok(())
}

fn say_bye() -> &'static str {
    "Bye!"
}
