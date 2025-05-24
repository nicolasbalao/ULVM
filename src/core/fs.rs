use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::{platform::detect_plateform, verbose};

use super::archive::build_archive_name;

#[derive(Debug, Error)]
pub enum FsError {
    #[error("Erreur IO: {0}")]
    Io(#[from] std::io::Error),

    #[error("Home directory not found")]
    HomeDirNotFound,

    #[error("Shim exec not found")]
    ShimExecNotFound,
}

pub fn ensure_ulvm_home_dir() -> Result<PathBuf, FsError> {
    let dir = dirs::home_dir()
        .ok_or(FsError::HomeDirNotFound)?
        .join(".ulvm");
    ensure_dir(dir)
}
pub fn ensure_ulvm_bin_dir() -> Result<PathBuf, FsError> {
    let dir = ensure_ulvm_home_dir()?.join("bin");
    ensure_dir(dir)
}

pub fn ensure_node_dir() -> Result<PathBuf, FsError> {
    let dir = ensure_ulvm_home_dir()?.join("node");
    ensure_dir(dir)
}

pub fn ensure_node_versions_dir() -> Result<PathBuf, FsError> {
    let dir = ensure_node_dir()?.join("versions");
    ensure_dir(dir)
}

pub fn ensure_node_downloads_dir() -> Result<PathBuf, FsError> {
    let dir = ensure_node_dir()?.join("downloads");
    ensure_dir(dir)
}

pub fn ensure_ulvm_node_bin_dir() -> Result<PathBuf, FsError> {
    let dir = ensure_node_dir()?.join("bin");
    ensure_dir(dir)
}

pub fn ensure_dir(dir: PathBuf) -> Result<PathBuf, FsError> {
    if !dir.exists() {
        verbose!("🔧 Create folder : {}", dir.display());
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

pub fn bin_node_version_exec_dir(version: &str) -> Result<PathBuf, FsError> {
    let version_path = ensure_node_versions_dir()?.join(version);
    let plateform = detect_plateform();

    if plateform == "win" {
        return Ok(version_path);
    }

    Ok(version_path.join("bin"))
}

pub fn exec_node_file_path(version: &str, name: &str) -> Result<PathBuf, FsError> {
    let bin_path = bin_node_version_exec_dir(version)?;
    Ok(bin_path.join(name))
}

pub fn create_symlink_dir(link: &Path, target: &Path) -> Result<(), FsError> {
    if link.exists() {
        fs::remove_dir_all(link)?;
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(target, link)?;

    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(target, link)?;

    Ok(())
}

pub fn remove_symlink(link: &Path) -> Result<(), FsError> {
    if link.exists() {
        verbose!("Removing symlink: {}", link.display());
        fs::remove_dir_all(link)?;
    }
    Ok(())
}

pub fn remove_version_dir(version_path: &Path) -> Result<(), FsError> {
    verbose!("Removing version dir: {}", version_path.display());
    Ok(fs::remove_dir_all(version_path)?)
}

pub fn remove_archive(version: &str) -> Result<(), FsError> {
    let archive_path = ensure_node_downloads_dir()?.join(build_archive_name(version));
    if archive_path.exists() {
        verbose!("Removing archive: {}", archive_path.display());
        fs::remove_file(archive_path)?;
    }
    Ok(())
}
