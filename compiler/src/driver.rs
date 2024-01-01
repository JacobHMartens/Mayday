extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_session;

use rustc_driver::{Callbacks, Compilation, RunCompiler};
use rustc_interface::{interface, Queries};
use rustc_session::{EarlyErrorHandler, config::ErrorOutputType};

use std::env;

use crate::{collector::UnsafeCollector, reporter::UnsafeReporter};

struct CustomCallbacks;

impl Callbacks for CustomCallbacks {

    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            let mut collector = UnsafeCollector { 
                tcx,
                functions: vec![],
                traits: vec![],
                impls: vec![],
                blocks: vec![], 
            };

            tcx.hir().visit_all_item_likes_in_crate(&mut collector);
            collector.report_unsafe_code();            
            
        });
        Compilation::Stop
    }
}


pub fn compile() {
    // Same handler and args as used in the main() function of lib.rs in rustc_driver_impl
    let handler = EarlyErrorHandler::new(ErrorOutputType::default());
    let args = env::args_os()
            .enumerate()
            .map(|(i, arg)| {
                arg.into_string().unwrap_or_else(|arg| {
                    handler.early_error(format!("argument {i} is not valid Unicode: {arg:?}"))
                })
            })
            .collect::<Vec<String>>();

    let mut callbacks = CustomCallbacks;

    let compiler = RunCompiler::new(&args, &mut callbacks);
    let _ = compiler.run();
}
