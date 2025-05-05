#[cfg(unix)]
mod unix;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(unix)]
use unix as platform;

#[cfg(target_os = "windows")]
use windows as platform;

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    platform::run()
}
