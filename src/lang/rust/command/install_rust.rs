use std::{io, process::Command};

use thiserror::Error;

use crate::{
    core::fs::FsError,
    info,
    lang::rust::{fs_rust::install_rust_common_dirs, is_rust_installed},
    success,
};

#[derive(Debug, Error)]
pub enum InstallRustError {
    #[error("Erreur FS: {0}")]
    Fs(#[from] FsError),

    #[error("Erreur d'exécution du processus: {0}")]
    ProcessIo(#[from] io::Error),

    #[error("La commande rustup a échoué avec le code de sortie: {0}")]
    RustupFailed(i32),

    #[error("Rust n'est pas installé, exécutez d'abord `ulvm rust install`")]
    RustNotInstalled,
}

// TODO: make wrapper for using commands with error handling avoid duplicate code

pub fn execute(version: Option<String>) -> Result<(), InstallRustError> {
    match version {
        Some(ver) => install_rust_version(&ver),
        None => install_rust_toolchain(),
    }
}

fn install_rust_version(version: &str) -> Result<(), InstallRustError> {
    let (rustup_home, cargo_home) = install_rust_common_dirs()?;

    if !is_rust_installed().unwrap_or(false) {
        return Err(InstallRustError::RustNotInstalled);
    }

    info!("Installing Rust toolchain version: {}", version);
    let status = Command::new("sh")
        .env("RUSTUP_HOME", &rustup_home)
        .env("CARGO_HOME", &cargo_home)
        .arg("-c")
        .arg(format!("rustup install {}", version))
        .status()?;

    if !status.success() {
        return Err(InstallRustError::RustupFailed(status.code().unwrap_or(-1)));
    }

    success!("Rust toolchain version {} installed successfully", version);
    Ok(())
}

fn install_rust_toolchain() -> Result<(), InstallRustError> {
    let (rustup_home, cargo_home) = install_rust_common_dirs()?;

    info!("Installing rustup...");

    let status = Command::new("sh")
        .env("RUSTUP_HOME", &rustup_home)
        .env("CARGO_HOME", &cargo_home)
        .arg("-c")
        .arg("curl https://sh.rustup.rs -sSf | sh -s -- -y")
        .status()?;

    if !status.success() {
        return Err(InstallRustError::RustupFailed(status.code().unwrap_or(-1)));
    }
    success!("Rustup installed successfully in {}", rustup_home.display());

    Ok(())
}
