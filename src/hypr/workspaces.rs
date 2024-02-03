use hyprland::data::*;
use hyprland::prelude::*;
use hyprland::shared::WorkspaceType;
use indexmap::IndexMap;
use serde::Serialize;

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

impl Wrkspcs {
    pub fn add(id: WorkspaceType) {
        let mut wrkspcs = Wrkspcs::get();
        wrkspcs.update_visible();

        let id = i32::from_workspacetype(id);
        let wrkspc = wrkspcs.workspaces.get_mut(&id).unwrap();
        wrkspc.active = true;

        println!("{}", serde_json::to_string(&wrkspcs).unwrap());
    }

    pub fn destroy(id: WorkspaceType) {
        let mut wrkspcs = Wrkspcs::get();
        wrkspcs.update_visible();

        let id = i32::from_workspacetype(id);
        let wrkspc = wrkspcs.workspaces.get_mut(&id).unwrap();
        wrkspc.active = false;

        println!("{}", serde_json::to_string(&wrkspcs.workspaces).unwrap());
    }

    pub fn change() {
        let mut wrkspcs = Wrkspcs::get();
        wrkspcs.update_visible();

        println!("{}", serde_json::to_string(&wrkspcs.workspaces).unwrap());
    }

    pub fn print_initial_wrkspcs() {
        let wrkspcs = Wrkspcs::get();
        println!("{}", serde_json::to_string(&wrkspcs.workspaces).unwrap());
    }
    fn get() -> Wrkspcs {
        let workspaces = Workspaces::get().unwrap().to_vec();
        let minified_workspaces = Wrkspcs::from_vec(workspaces);

        Wrkspcs {
            workspaces: minified_workspaces,
        }
    }

    fn from_vec(workspaces: Vec<Workspace>) -> IndexMap<i32, Wrkspc> {
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

    fn update_visible(&mut self) {
        let monitors = Monitors::get().unwrap().to_vec();
        let mut visible_workspaces: Vec<i32> = vec![];

        for monitor in &monitors {
            visible_workspaces.push(monitor.active_workspace.id);
        }

        for wrkspc in &mut self.workspaces {
            if visible_workspaces.contains(wrkspc.0) {
                wrkspc.1.visible = true;
            } else {
                wrkspc.1.visible = false;
            }
        }
    }
}

pub trait I32Ex {
    fn from_workspacetype(id: WorkspaceType) -> i32;
}

impl I32Ex for i32 {
    fn from_workspacetype(id: WorkspaceType) -> i32 {
        let id = match id.into() {
            hyprland::shared::WorkspaceType::Regular(id) => id.parse::<i32>().unwrap(),
            hyprland::shared::WorkspaceType::Special(Some(id)) => id.parse::<i32>().unwrap(),
            hyprland::shared::WorkspaceType::Special(None) => 0,
        };
        id
    }
}

// try to implement like this
// impl From<WorkspaceType> for i32 {}
