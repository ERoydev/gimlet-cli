# Gimlet CLI for Gimlet VS Code extension

A CLI tool for launching `lldb` with enhanced Rust + Solana formatters — built to simplify debugging Solana Anchor programs.

---

## Features

- Automatically injects **LLDB Rust + Solana** formatting scripts
- No global installs: runs everything in an isolated **temporary directory**
- Friendly CLI: just pass your compiled binary and debug
- Integrated with VS Code / terminal workflows

---

## Requirements
- Rust(stable)
- LLDB >= `20.1.7` (tested)
- Recommended: Install via `Homebrew` (on macOS)

## Installation

```sh
cargo install gimlet-cli
```

## Usage

```sh
gimlet-cli <absolute_path_to_binary>
```

1. Creates a temp sandbox.
2. Extract the LLDB formatters and scripts.
3. Launch `lldb` with everything configured.

## Notes
- We use this to run the `lldb` scripts with rust formatters instead of using `solana-lldb` which uses older LLDB version that have issues for our debugging goals.