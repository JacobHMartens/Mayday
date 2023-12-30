#![feature(rustc_private)]

mod driver;
mod queries;

fn main() {
    driver::compile();
}