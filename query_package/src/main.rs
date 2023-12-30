#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;

use std::time::SystemTime;

pub mod compiler;
pub mod tool;

fn main() {
    let start = SystemTime::now();

    tool::launch();
    
    let execution_time = start.elapsed();
    println!("{:?}", execution_time);
}
