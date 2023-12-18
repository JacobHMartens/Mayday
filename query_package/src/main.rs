#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;

use rustc_driver::RunCompiler;

pub mod queries;
pub mod callbacks;
pub mod cli;

fn main() {
    let mut args: Vec<String> = vec!["rustc".to_string()];
    args.extend_from_slice(&cli::get_all_args());
    let mut callbacks = callbacks::CustomCallbacks;
    let run_compiler = RunCompiler::new(&args, &mut callbacks);
    let _ = run_compiler.run();
}
