use std::sync::atomic::{AtomicBool, Ordering};

use colored::Colorize;

use crate::lang::node::version::NodeVersion;

/// Icônes standardisées
pub const ICON_SUCCESS: &str = "✅️";
pub const ICON_ERROR: &str = "❌";
pub const ICON_WARN: &str = "⚠️";
pub const ICON_INFO: &str = "ℹ️";
pub const ICON_ACTIVE: &str = "➡️";

static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn set_verbose(val: bool) {
    VERBOSE.store(val, Ordering::Relaxed);
}

pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}

pub fn success(msg: &str) {
    println!("{:<3} {} \n", ICON_SUCCESS, msg.green());
}

pub fn error(msg: &str) {
    eprintln!("{:<3} {} \n", ICON_ERROR, msg.red());
}

pub fn warn(msg: &str) {
    println!("{:<3}  {} \n", ICON_WARN, msg.yellow());
}

pub fn info(msg: &str) {
    println!("{:<3}  {} \n", ICON_INFO, msg.cyan());
}

pub fn verbose(msg: &str) {
    if is_verbose() {
        println!("{:<3}", msg.italic());
    }
}

// TODO refactor this
pub fn display_versions(versions: Vec<&NodeVersion>) {
    println!(
        "\n{:<12} {:<12} {:<10} {}",
        "Version".bold().cyan(),
        "Date".bold().cyan(),
        "Status".bold().cyan(),
        "Codename".bold().cyan()
    );
    println!();

    for version in &versions {
        if version.is_installed {
            println!(
                "{:<12} {:<12} {:<10} {}",
                version.version.cyan(),
                version.date.cyan(),
                version.status.cyan(),
                version.lts.clone().unwrap_or_default().cyan()
            );
        } else {
            println!(
                "{:<12} {:<12} {:<10} {}",
                version.version,
                version.date,
                version.status,
                version.lts.clone().unwrap_or_default()
            );
        }
    }
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        $crate::ui::success(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::ui::error(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::ui::warn(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::ui::info(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)*) => {
        $crate::ui::verbose(&format!($($arg)*))
    };
}
