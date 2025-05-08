use std::env;
use std::path::{Path, PathBuf};

use crate::{error, success};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 ULVM Setup - Environment Check (Windows)\n");

    let bin_dir = get_ulvm_bin_dir();
    check_directory_exists(&bin_dir);
    check_binaries_exist(&bin_dir);
    check_path_contains(&bin_dir)?;

    Ok(())
}

fn get_ulvm_bin_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(".ulvm").join("bin")
}

fn check_directory_exists(path: &Path) {
    if path.exists() {
        success!(".ulvm\\bin directory: {}", path.display());
    } else {
        error!(".ulvm\\bin directory: {}", path.display());
    }
}

fn check_binaries_exist(bin_dir: &Path) {
    let ulvm = bin_dir.join("ulvm.exe");
    let shim = bin_dir.join("ulvm_shim.exe");

    if ulvm.exists() && shim.exists() {
        success!("Binaries in place: ulvm.exe & ulvm_shim.exe");
    } else {
        error!("Binaries in place: ulvm.exe & ulvm_shim.exe");
    }
}

fn check_path_contains(bin_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Checking PATH environment variable...");
    let path_var = env::var_os("PATH").ok_or("Missing PATH variable")?;
    let mut paths = env::split_paths(&path_var);
    if paths.any(|p| p == *bin_dir) {
        success!(".ulvm\\bin is already in PATH.");
    } else {
        error!(".ulvm\\bin is NOT in PATH.");
        println!("👉 To use ulvm from anywhere, add this to your PATH:");
        println!(
            "   PowerShell: $env:PATH = \"{};\" + $env:PATH",
            bin_dir.display()
        );
        println!("   CMD:        set PATH={};%PATH%", bin_dir.display());
    }
    Ok(())
}
