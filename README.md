# BachelorProject
Bachelor Project - Building a tool for analysing unsafe code in Rust

## Workspace setup
The required toolchain and components will be downloaded and installed automatically 
when running one of the scripts in the project. To test that everything works as intended, try running one of the [examples](run-query-examples).

The following setup is only necessary if the compilation fails when running one of the project scripts.

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
To execute an example, run the following after replacing <example_file> with the name of the example file.
```bash
cargo run --example <example_file>
```
For example, the following executes the example query_rustc_driver.
```bash
cargo run --example query_rustc_driver
# Expected output:
#   Function: add_to_count#0
#   Function: main#0
```



