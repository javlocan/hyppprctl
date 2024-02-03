use hyprland::data::*;
use hyprland::prelude::*;
use hyprland::shared::WorkspaceType;
use indexmap::IndexMap;
use serde::Serialize;
// use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Wrkspc {
    active: bool,
    visible: bool,
    monitor: String,
}

#[derive(Debug, Serialize)]
pub struct Wrkspcs {
    pub workspaces: IndexMap<i32, Wrkspc>,
}

pub fn add(id: WorkspaceType) {
    let mut wrkspcs = get_wrkspcs();
    update_visible(&mut wrkspcs);

    let id = listener_id_to_i32(id);
    let wrkspc = wrkspcs.workspaces.get_mut(&id).unwrap();
    wrkspc.active = true;

    println!("{}", serde_json::to_string(&wrkspcs.workspaces).unwrap());
}

pub fn destroy(id: WorkspaceType) {
    let mut wrkspcs = get_wrkspcs();
    update_visible(&mut wrkspcs);

    let id = listener_id_to_i32(id);
    let wrkspc = wrkspcs.workspaces.get_mut(&id).unwrap();
    wrkspc.active = false;

    println!("{}", serde_json::to_string(&wrkspcs.workspaces).unwrap());
}

pub fn change() {
    let mut wrkspcs = get_wrkspcs();
    update_visible(&mut wrkspcs);

    println!("{}", serde_json::to_string(&wrkspcs.workspaces).unwrap());
}

pub fn print_initial_wrkspcs() {
    let wrkspcs = get_wrkspcs();
    println!("{}", serde_json::to_string(&wrkspcs.workspaces).unwrap());
}

fn get_wrkspcs() -> Wrkspcs {
    let workspaces = Workspaces::get().unwrap().to_vec();
    let minified_workspaces = minify_workspaces(workspaces);
    return Wrkspcs {
        workspaces: minified_workspaces,
    };
}

fn update_visible(wrkspcs: &mut Wrkspcs) {
    let monitors = Monitors::get().unwrap().to_vec();
    let mut visible_workspaces: Vec<i32> = vec![];

    for monitor in &monitors {
        visible_workspaces.push(monitor.active_workspace.id);
    }

    for wrkspc in &mut wrkspcs.workspaces {
        if visible_workspaces.contains(wrkspc.0) {
            wrkspc.1.visible = true;
        } else {
            wrkspc.1.visible = false;
        }
    }
    // for id in &visible_workspaces {
    //     wrkspcs.workspaces.get_mut(&id).unwrap().visible = true;
    // }
}

fn minify_workspaces(workspaces: Vec<Workspace>) -> IndexMap<i32, Wrkspc> {
    let monitors = Monitors::get().unwrap().to_vec();
    let mut visible_workspaces: Vec<i32> = vec![];
    for monitor in &monitors {
        visible_workspaces.push(monitor.active_workspace.id);
    }

    let mut minified_initial_workspaces = IndexMap::new();
    for id in 1..=10 {
        minified_initial_workspaces.insert(id, Wrkspc::default());
    }

    for workspace in workspaces {
        let id = workspace.id;
        let wrkspc = minified_initial_workspaces.get_mut(&id).unwrap();

        wrkspc.active = true;
        wrkspc.monitor = workspace.monitor;

        if visible_workspaces.contains(&id) {
            wrkspc.visible = true
        }
    }

    return minified_initial_workspaces;
}

fn listener_id_to_i32(id: WorkspaceType) -> i32 {
    let id = match id {
        hyprland::shared::WorkspaceType::Regular(id) => id.parse::<i32>().unwrap(),
        hyprland::shared::WorkspaceType::Special(Some(id)) => id.parse::<i32>().unwrap(),
        hyprland::shared::WorkspaceType::Special(None) => 0,
    };
    id
}
