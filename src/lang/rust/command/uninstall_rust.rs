use crate::{
    lang::rust::{RustCliErr, rust_cli_command},
    success,
};

pub fn execute(version: &str) -> Result<(), RustCliErr> {
    rust_cli_command(format!("rustup toolchain uninstall {}", &version).as_str())?;
    success!("Rustup toolchain {} is uninstalled", version);
    Ok(())
}
