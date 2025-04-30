use std::{
    ffi::OsString,
    fs::{self},
    io,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use thiserror::Error;

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

pub fn create_exec_symlink(version_path: &Path) -> Result<(), FsError> {
    let bin_dir = version_path.join("bin");
    let exec_names = find_executables(&bin_dir)?;

    let ulvm_bin_dir = ensure_ulvm_bin_dir()?;
    let shim_exec = ulvm_bin_dir.join("ulvm_shim");

    if !shim_exec.exists() {
        return Err(FsError::ShimExecNotFound);
    }

    for name in exec_names {
        let symlink_link = ulvm_bin_dir.join(&name);

        if !symlink_link.exists() {
            println!(
                "Creating symlink for exec {} to {}",
                name.into_string().unwrap(),
                symlink_link.display()
            );
            std::os::unix::fs::symlink(&shim_exec, &symlink_link)?;
        }
    }

    Ok(())
}

pub fn find_executables(dir: &Path) -> io::Result<Vec<OsString>> {
    let mut executables_names = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(&path)?;

            let permissions = metadata.permissions();

            // UNIX
            if permissions.mode() & 0o111 != 0 {
                executables_names.push(path.file_name().unwrap().to_owned());
            }
        }
    }

    Ok(executables_names)
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
