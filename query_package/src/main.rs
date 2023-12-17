#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;

use rustc_driver::RunCompiler;

pub mod queries;
pub mod callbacks;

fn main() {
    let args: Vec<String> = vec![r"rustc".to_string(), r"unsafe_example/src/main.rs".to_string()];
    let mut callbacks = callbacks::CustomCallbacks;
    let run_compiler = RunCompiler::new(&args, &mut callbacks);
    let _ = run_compiler.run();
}
