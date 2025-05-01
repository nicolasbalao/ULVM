use std::{
    ffi::OsString,
    fs::{self},
    io,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::platform::detect_plateform;

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

pub fn create_exec_symlink(version_path: &Path) -> Result<(), FsError> {
    let bin_dir = if cfg!(unix) {
        version_path.join("bin")
    } else {
        version_path.to_path_buf() // Windows
    };
    let exec_names = find_executables(&bin_dir)?;

    let ulvm_bin_dir = ensure_ulvm_bin_dir()?;
    let shim_exec = shim_exec_full_path()?;

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

            create_symlink(&shim_exec, &symlink_link)?;
        }
    }

    Ok(())
}

pub fn find_executables(dir: &Path) -> io::Result<Vec<OsString>> {
    let mut executables_names = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if is_executable(&path)? {
            if let Some(name) = path.file_name() {
                executables_names.push(name.to_owned());
            }
        }
    }

    Ok(executables_names)
}

pub fn remove_symlink_for_version(version: &str) -> Result<(), FsError> {
    let exec_names = find_executables(bin_node_version_exec_dir(version)?.as_path())?;
    let ulvm_bin_dir = ensure_ulvm_bin_dir()?;

    for name in exec_names {
        let bin_path = ulvm_bin_dir.join(&name);
        if bin_path.exists() {
            println!("Removing symlink: {}", bin_path.display());
            fs::remove_file(&bin_path)?;
        }
    }
    Ok(())
}

pub fn remove_version_dir(version_path: &Path) -> Result<(), FsError> {
    println!("Removing version dir: {}", version_path.display());
    Ok(fs::remove_dir_all(version_path)?)
}

pub fn remove_archive(version: &str) -> Result<(), FsError> {
    let archive_path = ensure_node_downloads_dir()?.join(build_archive_name(version));
    if archive_path.exists() {
        println!("Removing archive: {}", archive_path.display());
        fs::remove_file(archive_path)?;
    }
    Ok(())
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

#[cfg(unix)]
pub fn create_symlink<P, Q>(original: P, link: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    std::os::unix::fs::symlink(original, link)
}

// TODO: see for refactor all with hardLink
#[cfg(windows)]
pub fn create_symlink<P, Q>(original: P, link: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let original_path = original.as_ref();
    // if original_path.is_dir() {
    //     std::os::windows::fs::symlink_dir(original_path, link)
    // } else {
    //     std::os::windows::fs::symlink_file(original_path, link)
    // }

    fs::hard_link(original_path, link)
}

fn shim_exec_full_path() -> Result<PathBuf, FsError> {
    let bin_dir = ensure_ulvm_bin_dir()?;

    let ext = if cfg!(target_family = "windows") {
        ".exe"
    } else {
        ""
    };

    Ok(bin_dir.join(format!("ulvm_shim{}", ext)))
}

#[cfg(unix)]
fn is_executable(path: &Path) -> io::Result<bool> {
    std::os::unix::fs::PermissionsExt;
    let metadata = fs::metadata(path)?;
    let permissions = metadata.permissions();
    Ok(metadata.is_file() && (permissions.mode() & 0o111 != 0));
}

#[cfg(windows)]
fn is_executable(path: &Path) -> io::Result<bool> {
    let metadata = fs::metadata(path)?;

    if !metadata.is_file() {
        return Ok(false);
    }

    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext = ext.to_ascii_lowercase();
        Ok(matches!(ext.as_str(), "exe" | "bat" | "cmd"))
    } else {
        Ok(false)
    }
}
