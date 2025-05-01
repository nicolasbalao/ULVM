use clap::{Args, Subcommand, command};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct NodeArgs {
    #[command(subcommand)]
    pub command: NodeCommands,
}

#[derive(Subcommand, Debug)]
pub enum NodeCommands {
    Install {
        #[arg()]
        version: String,
    },
    Use {
        #[arg()]
        version: String,
    },
    List {
        #[arg(short, long)]
        remote: bool,

        #[arg(short, long)]
        all: bool,
    },
    Uninstall {
        #[arg()]
        version: String,
        #[arg(long)]
        hard: bool,
    },
}
