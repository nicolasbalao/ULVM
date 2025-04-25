pub mod node;
use clap::{Parser, Subcommand, command};
use node::NodeArgs;

#[derive(Parser, Debug)]
#[command(name = "ULVM", version, about = "Version manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Node(NodeArgs),
}
