use hyprland::data::*;
use hyprland::prelude::*;
use hyprland::shared::WorkspaceType;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Wrkspc {
    active: bool,
    visible: bool,
    monitor: String,
}

#[derive(Debug, Serialize)]
pub struct Wrkspcs {
    pub workspaces: HashMap<i32, Wrkspc>,
}

pub fn add(id: WorkspaceType) {
    let id = listener_id_to_i32(id);
    let mut wrkspcs = get_wrkspcs();
    let wrkspc = wrkspcs.workspaces.get_mut(&id).unwrap();

    wrkspc.active = true;

    println!("{}", serde_json::to_string(&wrkspcs.workspaces).unwrap());
}
pub fn destroy(id: WorkspaceType) {
    let id = listener_id_to_i32(id);
    let mut wrkspcs = get_wrkspcs();
    let wrkspc = wrkspcs.workspaces.get_mut(&id).unwrap();
    update_visible(&mut wrkspcs);
    wrkspc.active = false;

    println!("{}", serde_json::to_string(&wrkspcs.workspaces).unwrap());
}

pub fn get_wrkspcs() -> Wrkspcs {
    let workspaces = Workspaces::get().unwrap().to_vec();
    let minified_workspaces = minify_workspaces(workspaces);
    return Wrkspcs {
        workspaces: minified_workspaces,
    };
}

pub fn update_visible(wrkspcs: &mut Wrkspcs) {
    let monitors = Monitors::get().unwrap().to_vec();
    let mut visible_workspaces: Vec<i32> = vec![];
    for monitor in &monitors {
        visible_workspaces.push(monitor.active_workspace.id);
    }
    for wrkspc in wrkspcs.workspaces {
        if visible_workspaces.contains(&wrkspc.0) {
            let wrkspc = &wrkspc.1;
            wrkspc.visible = true
        }
    }
}

fn minify_workspaces(workspaces: Vec<Workspace>) -> HashMap<i32, Wrkspc> {
    let monitors = Monitors::get().unwrap().to_vec();
    let mut visible_workspaces: Vec<i32> = vec![];
    for monitor in &monitors {
        visible_workspaces.push(monitor.active_workspace.id);
    }

    let mut minified_initial_workspaces = HashMap::new();
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
