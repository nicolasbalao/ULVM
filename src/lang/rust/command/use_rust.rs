use thiserror::Error;

use crate::{
    info,
    lang::rust::{RustupErr, RustupNotInstallErr, is_rust_installed, rustup_cli},
    success,
};

#[derive(Debug, Error)]
pub enum UseRustErr {
    #[error("Error: {0}")]
    RustCli(#[from] RustupErr),

    #[error("Error: {0}")]
    RustupNotInstal(#[from] RustupNotInstallErr),
}

pub fn execute(version: &str) -> Result<(), UseRustErr> {
    info!("Setting up rust {} as current", version);
    is_rust_installed()?;

    rustup_cli(["default", version])?;
    success!("Rust {} is now the default version", version);
    Ok(())
}
