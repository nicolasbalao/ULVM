use std::path::PathBuf;

use crate::core::fs::{FsError, ensure_dir, ensure_ulvm_home_dir};

fn ensure_rust_home_dir() -> Result<PathBuf, FsError> {
    let dir = ensure_ulvm_home_dir()?.join("rust");
    ensure_dir(dir)
}

fn ensure_rustup_home_dir() -> Result<PathBuf, FsError> {
    let dir = ensure_rust_home_dir()?.join("rustup");
    ensure_dir(dir)
}

fn ensure_cargo_home_dir() -> Result<PathBuf, FsError> {
    let dir = ensure_rust_home_dir()?.join("cargo");
    ensure_dir(dir)
}

pub fn install_rust_common_dirs() -> Result<(PathBuf, PathBuf), FsError> {
    Ok((ensure_rustup_home_dir()?, ensure_cargo_home_dir()?))
}
