use pipewire::{registry::GlobalObject, spa::ForeignDict, Context, MainLoop, Properties};
use std::process::Command;

impl Vlm {
    // -----------------------
    /// Public functions to be called
    // -----------------------
    pub fn listen() -> Result<(), Box<dyn std::error::Error>> {
        let mainloop = MainLoop::new()?;
        let context = Context::new(&mainloop)?;
        let core = context.connect(None)?;
        let registry = core.get_registry()?;

        let _listener = registry
            .add_listener_local()
            // .global(|global| println!("New global: {:?}", global))
            .global(|global| Self::print_vlm(global))
            .register();

        mainloop.run();

        Ok(())
    }
    fn print_vlm(global: &GlobalObject<ForeignDict>) {
        println!("New global: {:?}", global.props);
    }
    pub fn set(args: Vec<String>) -> () {
        let action = match args[0].as_str() {
            "-i" | "--increase" => "+",
            "-d" | "--decrease" => "-",
            _ => panic!("Argument doesn't exist"),
        };
        let amount = &args[1];
        let amount = String::from(amount.to_owned() + "%" + action);

        let mut volume_cmd = Command::new("wpctl");
        volume_cmd
            .arg("set-volume")
            .arg("@DEFAULT_AUDIO_SINK@")
            .arg(amount)
            .spawn()
            .unwrap();
    }
}
// -----------------------
/// Structs and trait extensions
// -----------------------

pub struct Vlm {
    label: String,
    value: i32,
}

trait VolumeLabel {
    fn get_volume_label(value: i32) -> String;
}

impl VolumeLabel for String {
    fn get_volume_label(value: i32) -> String {
        match value {
            0..=20 => "so low",
            21..=40 => "low",
            41..=60 => "med",
            61..=80 => "high",
            81..=100 => "so high",
            101..=140 => "omg",
            _ => panic!("Volume value is out of bounds"),
        }
        .to_string()
    }
}
