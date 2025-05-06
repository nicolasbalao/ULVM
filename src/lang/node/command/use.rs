use std::env;

use thiserror::Error;

use crate::{
    core::{
        config::{
            errors::UlvmConfigError,
            ulvm_config::{NodeConfig, UlvmConfig},
        },
        fs::{
            self as ulvm_fs, FsError, create_exec_symlink, ensure_node_versions_dir,
            ensure_ulvm_bin_dir,
        },
    },
    platform, ui,
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
        ui::info(
            format!(
                "Node.js version {} not found locally. Installing...",
                version
            )
            .as_str(),
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
            ui::info(format!("Node.js version {} is already set as current.", version).as_str());
            return Ok(());
        }
    }

    // Met à jour la config
    config.node = Some(NodeConfig {
        version: version.to_string(),
    });

    config.save()?;

    let version_path = ensure_node_versions_dir()?.join(version);
    ui::info("Creating symlink");
    create_exec_symlink(&version_path)?;

    ui::success(format!("Now using Node.js version: {}", version).as_str());

    let path_env_var = env::var("PATH").unwrap();

    let ulvm_bin_dir = ensure_ulvm_bin_dir()?;
    if !path_env_var.contains(ulvm_bin_dir.to_str().unwrap()) {
        ui::info(
            format!(
                "Dont forget to add {} to your $PATH",
                ulvm_bin_dir.display()
            )
            .as_str(),
        );
        if platform::detect_plateform() == "win" {
            ui::info(format!("$env:PATH = \"{};$env:PATH\"", ulvm_bin_dir.display()).as_str());
        } else {
            ui::info(format!("export PATH=\"{}:$PATH\"", ulvm_bin_dir.display()).as_str());
        }
    }
    Ok(())
}
