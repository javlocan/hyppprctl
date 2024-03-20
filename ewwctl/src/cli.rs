use clap::{Args, Parser, Subcommand, ValueEnum};
use std::net::UdpSocket;
use strum_macros::{Display, EnumString};

const SOCKET_ADDR: &str = "127.0.0.1:9000";

impl Action {
    pub fn from_msg(msg: &str) -> Action {
        let &collon = &msg.rfind(':').unwrap();
        let &equal = &msg.rfind('=').unwrap_or_else(move || msg.len());

        let module = &msg[..collon];
        let module = Module::from_str(module, true).unwrap();
        let event = &msg[collon + 1..equal];
        let event = Event::from_str(event, true).unwrap();
        let prop = &msg[equal + 1..];
        let prop = Prop::from_str(prop, true).unwrap();

        let debounce = prop == Prop::Debounce;

        Action {
            module,
            event,
            debounce,
        }
    }

    pub fn send_event(&self) {
        let prop = if &self.debounce == &true {
            "debounce"
        } else {
            "none"
        };
        let msg = format!("{:#?}:{:#?}={}", &self.module, &self.event, prop);

        let socket = UdpSocket::bind("0.0.0.0:0").expect("s");
        let _ = socket.send_to(msg.as_bytes(), SOCKET_ADDR);
    }
}

#[derive(Parser, Debug)]
#[command(name = "ewwctl")]
#[command(bin_name = "ewwctl")]
#[command(arg_required_else_help = true)]
#[command(subcommand_help_heading = "Daemon")]
#[command(subcommand_value_name = "START--HELP")]
#[command(subcommand_negates_reqs = true)]
#[command(next_help_heading = "Actions")]
pub struct Ewwctl {
    #[command(subcommand)]
    pub commands: Option<Commands>,
    #[command(flatten)]
    pub args: Option<Action>,
}
#[derive(Args, Debug, Clone)]
pub struct Action {
    /// Module that emmits the event
    /// *
    #[arg(verbatim_doc_comment)]
    #[arg(short, long)]
    pub module: Module,
    /// e - Emmits events for the socket to process
    /// Event types:
    /// Hover       <module>
    /// Hoverlost   <module>
    /// For more information: ewwctl -e -h
    /// *
    #[arg(verbatim_doc_comment)]
    #[arg(short, long)]
    pub event: Event,
    #[arg(verbatim_doc_comment)]
    #[arg(long)]
    /// Adds a delay to the event execution
    pub debounce: bool,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Starts the socket and listens to the output.
    #[command(allow_missing_positional = true)]
    Start,
    #[command(external_subcommand)]
    Eww(Vec<String>),
}
#[derive(Debug, Clone, ValueEnum, EnumString, Display, Eq, PartialEq, Hash)]
pub enum Event {
    #[strum(serialize = "hover")]
    Hover,
    // #[strum(to_string = "hoverlost")]
    #[strum(serialize = "hoverlost")]
    Hoverlost,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, EnumString, Display)]
pub enum Module {
    // #[strum(to_string = "volume")]
    #[strum(serialize = "volume")]
    Volume,
    // #[strum(to_string = "brightness")]
    #[strum(serialize = "brightness")]
    Brightness,
    // #[strum(to_string = "wifi")]
    #[strum(serialize = "wifi")]
    Wifi,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, EnumString, Display)]
pub enum Prop {
    // #[strum(to_string = "debounce")]
    #[strum(serialize = "debounce")]
    Debounce,
    // #[strum(to_string = "none")]
    #[strum(serialize = "none")]
    None,
}
