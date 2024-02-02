pub mod workspaces {

    use hyprland::data::*;
    use hyprland::prelude::*;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Default)]
    pub struct Wrkspc {
        active: bool,
        visible: bool,
        monitor: String,
    }

    #[derive(Debug)]
    pub struct Wrkspcs {
        pub workspaces: HashMap<i32, Wrkspc>,
    }

    pub fn get() -> Wrkspcs {
        let workspaces = Workspaces::get().expect("upsie").to_vec();
        let minified_workspaces = minify_workspaces(workspaces);
        return Wrkspcs {
            workspaces: minified_workspaces,
        };
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
}
