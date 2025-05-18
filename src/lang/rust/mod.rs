use std::{io, process::Command};

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
pub enum RustupErr {
    #[error("Rustup cli execution error {0}")]
    ProcessIo(#[from] io::Error),
    #[error("La commande rustup a échoué avec le code de sortie: {0}")]
    RustupFailed(i32),
    #[error("Erreur FS: {0}")]
    Fs(#[from] FsError),
}

pub fn rustup_cli<I, S>(args: I) -> Result<(), RustupErr>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let status = Command::new("rustup").args(args).status()?;

    if !status.success() {
        return Err(RustupErr::RustupFailed(status.code().unwrap_or(-1)));
    }
    Ok(())
}
