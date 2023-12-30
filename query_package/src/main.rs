#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;

pub mod compiler;
pub mod tool;

fn main() {
    tool::launch();
}
