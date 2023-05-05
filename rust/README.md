# Learning Rust

## Getting Started

### Installation

```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Installed the default installation, which includes `cargo`, `clippy`, `rust-std`, `rustc`, `rustfm`

- Metadata and toolchains installed into the Rustup home directory `/Users/michaelliu/.rustup` and can be modified with the `RUSTUP_HOME` environment variable
- commands added to Cargo's bin directory `/Users/michaelliu/.cargo/bin`
    - commands added to the PATH environment variable by modifying
        - `/Users/michaelliu/.profile`
        - `/Users/michaelliu/.zshenv`
- To configure your current shell, run: `source "$HOME/.cargo/env"`

### Cargo

- Initializing new projects
    ```shell
    cargo new <PROJECT_NAME>
    ```
- Running projects
    ```shell
    cargo run
    ```
- Running projects
    ```shell
    cargo run
    ```
