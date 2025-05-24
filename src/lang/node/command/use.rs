use thiserror::Error;

use crate::{
    core::{
        config::{
            errors::UlvmConfigError,
            ulvm_config::{NodeConfig, UlvmConfig},
        },
        fs::{
            self as ulvm_fs, FsError, create_symlink_dir, ensure_node_dir, ensure_node_versions_dir,
        },
    },
    info, success, verbose,
};

use super::install::{self, InstallError};

#[derive(Error, Debug)]
pub enum UseError {
    #[error("Error filesystem handling: {0}")]
    UlvmFs(#[from] FsError),

    #[error("Error with the config: {0}")]
    UlvmConfig(#[from] UlvmConfigError),

    #[error("Error while installation: {0}")]
    Installation(#[from] InstallError),

    #[error("Creating symlink failed: {0}")]
    SymlinkCreation(#[from] std::io::Error),
}

pub fn execute(version: &str) -> Result<(), UseError> {
    // Vérifie que le dossier d'installation de la version existe
    let version_installation_path = ulvm_fs::ensure_node_versions_dir()?.join(version);

    if !version_installation_path.exists() {
        verbose!(
            "Node.js version {} not found locally. Installing...",
            version
        );
        install::execute(version)?;
        // Une fois installée, relancer `execute` pour configurer l'usage
        return execute(version);
    }

    // Charge la config actuelle
    let mut config = UlvmConfig::load_base_or_create()?;

    // Vérifie si la version demandée est déjà utilisée
    if let Some(ref node_config) = config.node {
        if node_config.version == version {
            info!("Node.js version {} is already set as current.", version);
            return Ok(());
        }
    }

    // Met à jour la config
    config.node = Some(NodeConfig {
        version: version.to_string(),
    });

    config.save()?;

    let version_path = ensure_node_versions_dir()?.join(version);
    verbose!("Creating symlink");

    let ulvm_node_dir = ensure_node_dir()?.join("bin");
    create_symlink_dir(&ulvm_node_dir, &version_path.join("bin"))?;

    success!("Now using Node.js version: {}", version);

    Ok(())
}
