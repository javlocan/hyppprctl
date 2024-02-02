mod workspaces;

use crate::workspaces::workspaces::get;
use hyprland::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let args_amount: usize = args.len() - 1;

    if args_amount > 0 {
        println!("Hello, {}. This is hyprevents", args[1]);

        if args[1] == "Yayo" {
            println!("Heeeeeeeey {}, you're back!", args[1]);
            let wrkspcs = get();
            println!("{:?}", wrkspcs.workspaces);
        }

        // let mut listener = EventListener::new();
        //
        // // listener.add_workspace_change_handler(|get_workspaces()| println!("{get_workspaces()}"));
        // listener.add_workspace_added_handler(|id| println!("added: {id}"));
        // listener.add_workspace_destroy_handler(|id| println!("destroyed: {id}"));
        //
        // listener.start_listener().expect("upsy doopsie");
    } else {
        println!("{}", say_bye())
    }

    Ok(())
}

fn say_bye() -> &'static str {
    return "Bye!";
}
