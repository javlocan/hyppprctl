use std::env;

use hyprland::data::*;
use hyprland::prelude::*;

use hyprland::event_listener::EventListener;
use hyprland::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("{}", args.len());
    println!(
        "Hello, {}. Let's show you the workspaces, unless you are Yayo.",
        args[1]
    );

    if args[1] == "Yayo" {
        println!("Heeeeeeeey {}, you're back!", args[1]);
        println!("We're listening...");

        let mut listener = EventListener::new();

        // listener.add_workspace_change_handler(|get_workspaces()| println!("{get_workspaces()}"));
        listener.add_workspace_added_handler(|data| println!("added: {data}"));
        listener.add_workspace_destroy_handler(|id| println!("destroyed: {id}"));

        listener.start_listener().expect("upsy doopsie");
    } else {
        let workspaces = get_workspaces();
        println!("{workspaces:#?}");
    }

    Ok(())
}

fn get_workspaces() -> Box<&'static str> {
    let workspaces = Workspaces::get().expect("Cannot unwrap workspace").to_vec();
    let algo = "{workspaces:#?}";
    let output = Box::new("{workspaces:#?}");
    return output;
}
