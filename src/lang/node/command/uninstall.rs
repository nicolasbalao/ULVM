use thiserror::Error;

use crate::{
    core::{
        config::{errors::UlvmConfigError, ulvm_config::UlvmConfig},
        fs::{
            FsError, ensure_node_versions_dir, remove_archive, remove_symlink_for_version,
            remove_version_dir,
        },
    },
    ui,
};

#[derive(Error, Debug)]
pub enum UninstallError {
    #[error("Error filesystem handling: {0}")]
    Fs(#[from] FsError),

    #[error("Error config: {0}")]
    UlvmConfig(#[from] UlvmConfigError),

    #[error("Error with io: {0}")]
    Io(#[from] std::io::Error),
}

pub fn execute(version: &str, hard: bool) -> Result<(), UninstallError> {
    // Delete version directory
    let version_path = ensure_node_versions_dir()?.join(version);

    if !version_path.exists() {
        ui::info(format!("Nodejs {} is not installed", version).as_str());
        return Ok(());
    }

    // Check if current version
    let mut base_config = UlvmConfig::load_base()?;

    let is_current = base_config
        .node
        .as_ref()
        .map(|cfg| cfg.version == version)
        .unwrap_or(false);

    if is_current {
        ui::info(format!("Nodejs {} is your current version", &version).as_str());
        base_config.node = None;
        base_config.save()?;
        remove_symlink_for_version(version)?;
    }

    // If hard delete archive
    if hard {
        remove_archive(version)?;
    }

    remove_version_dir(&version_path)?;

    ui::info(format!("Nodejs {} is uninstalled", &version).as_str());

    Ok(())
}
