use std::process::Command;

pub mod command;
pub mod fs_rust;

pub fn is_rust_installed() -> Result<bool, std::io::Error> {
    let mut cmd = Command::new("sh");

    cmd.arg("-c").arg("rustup --version");

    Ok(cmd.status()?.success())
}
