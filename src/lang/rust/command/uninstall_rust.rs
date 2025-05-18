use crate::{
    lang::rust::{RustupErr, rustup_cli},
    success,
};

pub fn execute(version: &str) -> Result<(), RustupErr> {
    rustup_cli(["toolchain", "uninstall", version])?;
    success!("Rustup toolchain {} is uninstalled", version);
    Ok(())
}
