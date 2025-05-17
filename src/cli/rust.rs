use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct RustArgs {
    #[command(subcommand)]
    pub command: RustCommands,
}

#[derive(Subcommand, Debug)]
pub enum RustCommands {
    Install {
        /// Toolchain name, such as 'stable', 'nightly', or '1.8.0'.
        #[arg()]
        version: Option<String>,
    },
    List,
    Use {
        /// Toolchain name, such as 'stable', 'nightly', '1.8.0', or a custom toolchain name
        #[arg()]
        version: String,
    },
    Uninstall {
        #[arg()]
        version: String,
    },
}
