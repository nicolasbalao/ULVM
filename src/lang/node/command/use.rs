use thiserror::Error;

use crate::core::{
    config::{
        errors::UlvmConfigError,
        ulvm_config::{NodeConfig, UlvmConfig},
    },
    fs::{self as ulvm_fs, FsError},
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
}

pub fn execute(version: &str) -> Result<(), UseError> {
    // Vérifie que le dossier d'installation de la version existe
    let version_installation_path = ulvm_fs::ensure_node_versions_dir()?.join(version);

    if !version_installation_path.exists() {
        println!("Node.js version {version} not found locally. Installing...");
        install::execute(version)?;
        // Une fois installée, relancer `execute` pour configurer l'usage
        return execute(version);
    }

    // Charge la config actuelle
    let mut config = UlvmConfig::load_base_or_create()?;

    // Vérifie si la version demandée est déjà utilisée
    if let Some(ref node_config) = config.node {
        if node_config.version == version {
            println!("Node.js version {version} is already set as current.");
            return Ok(());
        }
    }

    // Met à jour la config
    config.node = Some(NodeConfig {
        version: version.to_string(),
    });

    config.save()?;

    println!("Now using Node.js version: {version}");

    Ok(())
}
