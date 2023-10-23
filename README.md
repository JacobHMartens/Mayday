# BachelorProject
Bachelor Project - Building a tool for analysing unsafe code in Rust

## Workspace setup

### Set the workspace toolchain to Nightly
```bash
rustup toolchain install nightly-2023-21-10
rustup override set nightly-2023-21-10

```

### Install the rust compiler library
Install the necessary toolchain components.
Run the following:
```bash
rustup component add rustc-dev --toolchain nightly-2023-21-10
rustup component add llvm-tools --toolchain nightly-2023-21-10
rustup component add rust-src --toolchain nightly-2023-21-10
```

## Run query examples
Run the following after replacing <example_file> with the name of the example file, e.g. query_rustc_driver.
```bash
cargo run --example <example_file>
```



