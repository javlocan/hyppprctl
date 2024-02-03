mod hypr;

use crate::hypr::*;
use hyprland::{event_listener::EventListener, Result};
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
            workspaces::print_initial_wrkspcs();

            let mut listener = EventListener::new();
            listener.add_workspace_added_handler(|id| workspaces::add(id));
            listener.add_workspace_destroy_handler(|id| workspaces::destroy(id));
            listener.add_workspace_change_handler(|_| workspaces::change());
            listener.start_listener().unwrap();
        }
    }
    Ok(())
}

fn say_bye() -> &'static str {
    "Bye!"
}
