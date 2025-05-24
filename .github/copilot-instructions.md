# Copilot Instructions for ULVM

## Project Summary

ULVM (Universal Language Version Manager) is a cross-platform CLI tool written in Rust. It aims to manage multiple language runtimes (e.g. Node.js, Rust, Go) across different versions.

The tool is designed to:

- Work consistently on both **Linux** and **Windows**
- Handle installation, switching, and removal of language versions
- Provide an intuitive command set: `list`, `install`, `use`, `uninstall`, `setup`
- Avoid reliance on external dependencies or shell-specific scripts
- Package as a standalone binary for ease of distribution

The goal is to offer a reliable and performant alternative to existing tools like `nvm`, `rustup`, or `gvm`, but with a **unified interface**, no runtime dependencies, and full **cross-platform** support.

---

## Rust Best Practices to Follow

- Prefer **safe Rust**: avoid `unsafe` unless absolutely necessary.
- Use `Result<T, E>` and propagate errors properly (e.g. via `?`).
- Avoid `unwrap` and `expect` in non-test or non-critical code.
- Use `thiserror` for custom error types.
- Separate concerns: keep CLI code (e.g. argument parsing) distinct from core logic.
- Follow idiomatic module structure: split into `cli`, `core`, `utils`, etc.
- Implement traits and encapsulate logic in structs rather than using free-floating functions.
- Use `clap` (or `argh`) for CLI interface parsing with subcommand support.
- Write **unit tests** for core logic and **integration tests** for CLI flows.
- Use `cfg(target_os)` or similar guards to handle platform-specific behavior cleanly.
- Favor clear, descriptive names for functions and variables.
- Document public functions and structs with `///` comments.
- When writing cross-platform code, isolate OS-specific logic into dedicated modules.
