use std::env;
use std::path::{Path, PathBuf};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 ULVM Setup - Environment Check (Unix)\n");

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
    let status = if path.exists() { "✅️" } else { "❌" };
    println!("{:<2} .ulvm/bin directory: {}", status, path.display());
}

fn check_binaries_exist(bin_dir: &Path) {
    let ulvm = dirs::home_dir()
        .expect("Unable to find home dir")
        .join(".local")
        .join("bin");
    let shim = bin_dir.join("ulvm_shim");
    let status = if ulvm.exists() && shim.exists() {
        "✅️"
    } else {
        "❌"
    };
    println!(
        "{:<2} Binaries in place: {} & {}",
        status,
        ulvm.display(),
        shim.display()
    );
}

fn check_path_contains(bin_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Checking PATH environment variable...");
    let path_var = env::var_os("PATH").ok_or("Missing PATH variable")?;
    let mut paths = env::split_paths(&path_var);
    if paths.any(|p| p == *bin_dir) {
        println!("✅️ {} is already in PATH.", bin_dir.display());
    } else {
        println!("❌ .ulvm/bin is NOT in PATH.");
        println!("👉 Add the following to your shell profile (e.g., ~/.bashrc, ~/.zshrc):");
        println!("   export PATH=\"$HOME/.ulvm/bin:$PATH\"");
    }
    Ok(())
}
