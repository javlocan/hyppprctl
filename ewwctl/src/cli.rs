use clap::{Args, Parser, Subcommand, ValueEnum};
use strum_macros::EnumString;

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
    pub args: Option<Arguments>,
}
#[derive(Args, Debug)]
pub struct Arguments {
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
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Starts the socket and listens to the output.
    #[command(allow_missing_positional = true)]
    Start,
    #[command(external_subcommand)]
    Eww(Vec<String>),
}
#[derive(Debug, Clone, ValueEnum, EnumString)]
pub enum Event {
    #[strum(serialize = "hover")]
    Hover,
    #[strum(serialize = "hoverlost")]
    Hoverlost,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, EnumString)]
pub enum Module {
    #[strum(serialize = "volume")]
    Volume,
    #[strum(serialize = "brightness")]
    Brightness,
    #[strum(serialize = "wifi")]
    Wifi,
}
