use std::{fs, path::PathBuf};

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

pub fn find_rust_installed_versions() -> Result<Vec<String>, FsError> {
    let rustup_dir = ensure_rustup_home_dir()?;
    let toolchain_dir = rustup_dir.join("toolchains");

    let mut versions = Vec::new();

    for entry in fs::read_dir(toolchain_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|name| name.to_str()) {
                if let Some(version) = extract_version(dir_name) {
                    versions.push(version);
                }
            }
        }
    }

    Ok(versions)
}
// Fonction pour extraire la version du nom du dossier
fn extract_version(name: &str) -> Option<String> {
    // Vérifie si le nom commence par une version (par exemple "1.85.0")
    if let Some(pos) = name.find('-') {
        let version_prefix = &name[0..pos];
        if is_version_prefix(version_prefix) {
            return Some(version_prefix.to_string());
        }
    }

    None
}

// Fonction qui vérifie si une chaîne de caractères est un préfixe de version valide (comme 1.86.0)
fn is_version_prefix(name: &str) -> bool {
    name.chars().all(|c| c.is_numeric() || c == '.')
}
