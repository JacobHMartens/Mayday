#![feature(rustc_private)]

mod driver;
mod collector;
mod reporter;

fn main() {
    driver::compile();
}