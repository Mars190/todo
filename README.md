# Cross-Compiling Rust for Linux on Windows

This will walk you through the steps of building a binary for linux on a windows machine

## Requirements 
- Rust
- Running Docker Engine
- Cross, a Rust Crate 
    ```bash
    cargo install cross
    ```

## Steps
1. Add Linux Target for Rust
    ```bash
    rustup target add x86_64-unknown-linux-gnu
    ```
2. Cross compile
    ```bash
    cross build --target x86_64-unknown-linux-gnu --release
    ```