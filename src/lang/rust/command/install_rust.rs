use std::{fs, io, path::Path, process::Command};

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
    #[error("Fail to write env file for rust")]
    FailedToWriteEnvFile(io::Error),
}

pub fn execute(version: Option<String>) -> Result<(), InstallRustError> {
    match version {
        Some(ver) => install_rust_version(&ver),
        None => install_rust_toolchain(),
    }
}

fn install_rust_version(version: &str) -> Result<(), InstallRustError> {
    if !is_rust_installed().unwrap_or(false) {
        return Err(InstallRustError::RustNotInstalled);
    }

    info!("Installing Rust toolchain version: {}", version);
    let status = Command::new("rustup")
        .arg("install")
        .arg(version)
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
        .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path")
        .status()?;

    if !status.success() {
        return Err(InstallRustError::RustupFailed(status.code().unwrap_or(-1)));
    }

    write_rust_ulvm_env_file(&rustup_home, &cargo_home)
        .map_err(InstallRustError::FailedToWriteEnvFile)?;

    success!("Env file for rust successfully created");
    success!("Rustup installed successfully in {}", rustup_home.display());

    Ok(())
}

fn write_rust_ulvm_env_file(rustup_home: &Path, cargo_home: &Path) -> Result<(), std::io::Error> {
    let ulvm_rust_dir = rustup_home.parent().unwrap_or_else(|| Path::new("."));
    let env_file_path = ulvm_rust_dir.join("rust.env");

    let contents = format!(
        ". {cargo}/env\n\
        export RUSTUP_HOME={rustup}\n\
        export CARGO_HOME={cargo}",
        rustup = rustup_home.display(),
        cargo = cargo_home.display(),
    );

    fs::write(&env_file_path, contents)
}
