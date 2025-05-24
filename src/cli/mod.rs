pub mod node;
pub mod rust;
use clap::{Parser, Subcommand, command};
use node::{NodeArgs, NodeCommands};
use rust::{RustArgs, RustCommands};

use crate::{error, lang, ui::set_verbose};

#[derive(Parser, Debug)]
#[command(name = "ULVM", version, about = "Version manager")]
pub struct Cli {
    /// Enable verbose output to display additional details during execution
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Node(NodeArgs),
    /// Wrapper of rustup cli for managing versions
    Rust(RustArgs),
}

pub fn run() {
    let cli_arg = Cli::parse();

    set_verbose(cli_arg.verbose);

    match cli_arg.command {
        Commands::Node(node_args) => match node_args.command {
            NodeCommands::Install { version } => {
                if let Err(e) = lang::node::command::install::execute(&version) {
                    error!("{}", e);
                    std::process::exit(1);
                }
            }
            NodeCommands::Use { version } => {
                if let Err(e) = lang::node::command::r#use::execute(&version) {
                    error!("{}", e);
                    std::process::exit(1);
                }
            }
            NodeCommands::List { all } => {
                let result = if all {
                    lang::node::command::list::all_remote_execute()
                } else {
                    lang::node::command::list::remote_execute()
                };

                if let Err(e) = result {
                    error!("{}", e);
                    std::process::exit(1);
                }
            }
            NodeCommands::Uninstall { version, hard } => {
                if let Err(e) = lang::node::command::uninstall::execute(&version, hard) {
                    error!("{}", e);
                    std::process::exit(1);
                }
            }
        },
        Commands::Rust(rust_args) => match rust_args.command {
            RustCommands::Install { version } => {
                if let Err(e) = lang::rust::command::install_rust::execute(version) {
                    error!("{}", e);
                    std::process::exit(1)
                }
            }
            RustCommands::List => {
                lang::rust::command::list_rust::execute();
            }
            RustCommands::Use { version } => {
                if let Err(e) = lang::rust::command::use_rust::execute(&version) {
                    error!("{}", e);
                    std::process::exit(1)
                };
            }
            RustCommands::Uninstall { version } => {
                if let Err(e) = lang::rust::command::uninstall_rust::execute(&version) {
                    error!("{}", e);
                    std::process::exit(1)
                }
            }
        },
    }
}
