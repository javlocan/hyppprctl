pub mod workspaces {

    use hyprland::data::*;
    use hyprland::prelude::*;
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct Wrkspc {
        _active: bool,
        _visible: bool,
        _monitor: String,
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

    // pub fn add(mut l: EventListener) -> EventListener {
    //     l.add_workspace_change_handler(|id| println!("{id}"));
    //     return l;
    // }

    fn minify_workspaces(workspaces: Vec<Workspace>) -> HashMap<i32, Wrkspc> {
        let monitors = Monitors::get().expect("upsie").to_vec();
        let mut visible_workspaces: Vec<i32> = vec![];
        for monitor in &monitors {
            visible_workspaces.push(monitor.active_workspace.id);
        }

        let mut minified_initial_workspaces = HashMap::new();

        // let mut minified_initial_workspace = Box::new(Wrkspc {
        //     active: false,
        //     visible: false,
        //     monitor: String::from(""),
        // });
        let minified_initial_workspace = Wrkspc {
            _active: false,
            _visible: false,
            _monitor: String::from(""),
        };

        for id in 1..10 {
            let boxed_initial_workspace = minified_initial_workspace.clone();
            minified_initial_workspaces.insert(id, boxed_initial_workspace);
        }

        for workspace in workspaces {
            let visible = visible_workspaces.contains(&workspace.id);
            let wrkspc = Wrkspc {
                _active: true,
                _visible: visible,
                _monitor: workspace.monitor,
            };
            minified_initial_workspaces.insert(workspace.id, wrkspc);
        }
        return minified_initial_workspaces;
    }
}
