use std::{
    fs::{self},
    path::PathBuf,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FsError {
    #[error("Erreur IO: {0}")]
    Io(#[from] std::io::Error),

    #[error("Home directory not found")]
    HomeDirNotFound,
}

pub fn ensure_ulvm_home_dir() -> Result<PathBuf, FsError> {
    let dir = dirs::home_dir()
        .ok_or(FsError::HomeDirNotFound)?
        .join(".ulvm");
    ensure_dir(dir)
}

pub fn ensure_node_versions_dir() -> Result<PathBuf, FsError> {
    let dir = ensure_versions_dir()?.join("node");
    ensure_dir(dir)
}

pub fn ensure_node_downloads_dir() -> Result<PathBuf, FsError> {
    let dir = ensure_downloads_dir()?.join("node");
    ensure_dir(dir)
}

pub fn ensure_dir(dir: PathBuf) -> Result<PathBuf, FsError> {
    if !dir.exists() {
        println!("🔧 Create folder : {}", dir.display());
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

// ------------- PRIVATE ---------------
fn ensure_versions_dir() -> Result<PathBuf, FsError> {
    let dir = ensure_ulvm_home_dir()?.join("versions");
    ensure_dir(dir)
}

fn ensure_downloads_dir() -> Result<PathBuf, FsError> {
    let dir = ensure_ulvm_home_dir()?.join("downloads");
    ensure_dir(dir)
}
