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

## Build errors
If you get a build error like the following:\
``
error[E0433]: failed to resolve: could not find `provider` in `fallback`
--> rust\compiler\rustc_baked_icu_data\src\data\fallback\likelysubtags_v1\mod.rs:2:61
|
2 | type DataStruct = < :: icu_provider_adapters :: fallback :: provider :: LocaleFallbackLikelySubtagsV1Marker as :: icu_provider :: DataMar...
|                                                             ^^^^^^^^ could not find `provider` in `fallback
``\

Then run:

git submodule update --init --recursive

If it persists, then you are probably trying to compile and run with cargo. Use rustc to compile and rustup to run instead.
rustc main.rs 
\
rustup run nightly ./main

