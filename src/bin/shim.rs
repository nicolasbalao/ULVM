use std::{env, process::Command};

use ulvm::core::{config::ulvm_config::UlvmConfig, fs::exec_node_file_path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shim_name = env::args()
        .next()
        .and_then(|p| {
            std::path::Path::new(&p)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "unknown".into());

    let ulvm_config = UlvmConfig::load().unwrap_or_else(|e| {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    });

    let version = ulvm_config.node.unwrap().version;
    let binary_path = match shim_name.as_str() {
        "node" | "node.exe" => {
            // TODO handle if the version doesn't exist
            exec_node_file_path(&version, &shim_name)?
        }
        "npm" | "npm.cmd" => exec_node_file_path(&version, &shim_name)?,
        "npx" | "npx.cmd" => exec_node_file_path(&version, &shim_name)?,
        "corepack" | "corepack.cmd" => exec_node_file_path(&version, &shim_name)?,
        _ => panic!("Unsupported shim: {}", shim_name),
    };

    let args: Vec<String> = env::args().skip(1).collect();
    let status = Command::new(binary_path).args(args).status()?;

    std::process::exit(status.code().unwrap_or(1));
}
