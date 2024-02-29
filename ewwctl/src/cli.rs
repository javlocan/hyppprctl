use clap::{Args, Parser, Subcommand, ValueEnum};
use strum_macros::{Display, EnumString};

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
#[derive(Debug, Clone, ValueEnum, EnumString, Display)]
pub enum Event {
    #[strum(serialize = "hover", to_string = "hover")]
    Hover,
    #[strum(to_string = "hoverlost")]
    #[strum(serialize = "hoverlost")]
    Hoverlost,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, EnumString, Display)]
pub enum Module {
    #[strum(to_string = "volume")]
    #[strum(serialize = "volume")]
    Volume,
    #[strum(to_string = "brightness")]
    #[strum(serialize = "brightness")]
    Brightness,
    #[strum(to_string = "wifi")]
    #[strum(serialize = "wifi")]
    Wifi,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, EnumString, Display)]
pub enum Prop {
    #[strum(to_string = "debounce")]
    #[strum(serialize = "debounce")]
    Debounce,
    #[strum(to_string = "none")]
    #[strum(serialize = "none")]
    None,
}
