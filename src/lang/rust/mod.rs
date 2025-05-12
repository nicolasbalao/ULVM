use std::{io, process::Command};

use fs_rust::install_rust_common_dirs;
use thiserror::Error;

use crate::core::fs::FsError;

pub mod command;
pub mod fs_rust;

#[derive(Debug, Error)]
pub enum RustupNotInstallErr {
    #[error("Rustup cli is not installed")]
    NotInstalled(#[from] io::Error),
}
pub fn is_rust_installed() -> Result<bool, RustupNotInstallErr> {
    let mut cmd = Command::new("sh");

    cmd.arg("-c").arg("rustup --version");

    Ok(cmd.status()?.success())
}

#[derive(Debug, Error)]
pub enum RustCliErr {
    #[error("Rustup cli execution error {0}")]
    ProcessIo(#[from] io::Error),
    #[error("La commande rustup a échoué avec le code de sortie: {0}")]
    RustupFailed(i32),
    #[error("Erreur FS: {0}")]
    Fs(#[from] FsError),
}

pub fn rust_cli_command(command: &str) -> Result<(), RustCliErr> {
    let (rustup_home, cargo_home) = install_rust_common_dirs()?;
    let status = Command::new("sh")
        .env("RUSTUP_HOME", &rustup_home)
        .env("CARGO_HOME", &cargo_home)
        .arg("-c")
        .arg(command)
        .status()?;

    if !status.success() {
        return Err(RustCliErr::RustupFailed(status.code().unwrap_or(-1)));
    }
    Ok(())
}
