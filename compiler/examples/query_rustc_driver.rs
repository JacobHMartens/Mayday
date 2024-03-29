#![feature(rustc_private)]

/*
    Implementation using rustc driver
*/

extern crate rustc_interface;
extern crate rustc_driver;
extern crate rustc_hir;

use rustc_driver::{Callbacks, Compilation, RunCompiler};
use rustc_interface::{interface, Queries};
use rustc_hir::ItemKind;

pub struct CustomCallbacks;

impl Callbacks for CustomCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            let mut output: Vec<String> = vec![];
            for id in tcx.hir().items() {
                let item = tcx.hir().item(id);
                match item.kind {
                    ItemKind::Fn(_, _, _) => {
                        output.push(item.ident.to_string());
                        println!("Function: {:?}", item.ident);
                    }
                    _ => {}
                }
            }
            debug_assert_eq!(output, ["add_to_count", "main"]);
        });
        Compilation::Continue
    }
}

fn main() {
    let args: Vec<String> = vec![
        r"rustc".to_string(),
        r"unsafe_example/src/main.rs".to_string(),
        r"--out-dir".to_string(),
        r"target/debug/deps".to_string()
    ];
    let mut callbacks = CustomCallbacks;
    let run_compiler = RunCompiler::new(&args, &mut callbacks);
    let _ = run_compiler.run();
}
