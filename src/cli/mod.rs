pub mod command;
pub mod node;
use clap::{Parser, Subcommand, command};
use node::{NodeArgs, NodeCommands};

use crate::lang;

#[derive(Parser, Debug)]
#[command(name = "ULVM", version, about = "Version manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Node(NodeArgs),
    Setup,
}

pub fn run() {
    let cli_arg = Cli::parse();

    match cli_arg.command {
        Commands::Node(node_args) => match node_args.command {
            NodeCommands::Install { version } => {
                if let Err(e) = lang::node::command::install::execute(&version) {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
            NodeCommands::Use { version } => {
                if let Err(e) = lang::node::command::r#use::execute(&version) {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
            NodeCommands::List { remote, all } => {
                if remote && !all {
                    if let Err(e) = lang::node::command::list::remote_execute() {
                        eprintln!("{:?}", e);
                        std::process::exit(1);
                    }
                } else if remote && all {
                    if let Err(e) = lang::node::command::list::all_remote_execute() {
                        eprintln!("{:?}", e);
                        std::process::exit(1);
                    }
                } else {
                    println!("Listing local node.js versions...")
                }
            }
            NodeCommands::Uninstall { version, hard } => {
                if let Err(e) = lang::node::command::uninstall::execute(&version, hard) {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        },
        Commands::Setup => {
            if let Err(e) = command::setup::execute() {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
