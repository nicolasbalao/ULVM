use thiserror::Error;

use crate::{
    info,
    lang::rust::{RustCliErr, RustupNotInstallErr, is_rust_installed, rust_cli_command},
    success,
};

#[derive(Debug, Error)]
pub enum UseRustErr {
    #[error("Error: {0}")]
    RustCli(#[from] RustCliErr),

    #[error("Error: {0}")]
    RustupNotInstal(#[from] RustupNotInstallErr),
}

pub fn execute(version: &str) -> Result<(), UseRustErr> {
    info!("Setting up rust {} as current", version);
    rust_cli_command(format!("rustup default {}", version).as_str())?;
    is_rust_installed()?;
    success!("Rust {} is now the default version", version);
    Ok(())
}
