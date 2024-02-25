use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "ewwctl")]
#[command(bin_name = "ewwctl")]
#[command(arg_required_else_help = true)]
#[command(subcommand_help_heading = "Daemon")]
#[command(subcommand_value_name = "start - help")]
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
#[derive(Debug, Clone, ValueEnum)]
pub enum Event {
    Hover,
    Hoverlost,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Module {
    Volume,
    Brightness,
    Wifi,
}
