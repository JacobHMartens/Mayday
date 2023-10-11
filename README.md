# BachelorProject
Bachelor Project - Building a tool for analysing unsafe code in Rust

## Install rust compiler library
git clone https://github.com/rust-lang/rust.git

cd rust

Open powershell:

Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

./x build

## Set workspace environment to use Nightly
rustup toolchain install nightly

rustup override set nightly