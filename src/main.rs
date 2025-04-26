use clap::Parser;

mod cli;
mod core;
mod lang;

use cli::{Cli, Commands, node::NodeCommands};

fn main() {
    let cli_arg = Cli::parse();

    match cli_arg.command {
        Commands::Node(node_args) => match node_args.command {
            NodeCommands::Install { version } => {
                println!("Installing Node.js version {version}");
            }
            NodeCommands::Use { version } => {
                println!("Using Node.js version {version}");
            }
            NodeCommands::List { remote } => {
                if remote {
                    let _ = lang::node::command::list::remote_execute();
                } else {
                    println!("Listing local node.js versions...")
                }
            }
            NodeCommands::Uninstall { version } => {
                println!("Uninstall node.js version {version}")
            }
        },
    }
}
