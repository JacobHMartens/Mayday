#![feature(rustc_private)]

/*
    Implementation using rustc driver
*/

extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

pub struct CustomCallbacks;

impl rustc_driver::Callbacks for CustomCallbacks {
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        config.input = rustc_session::config::Input::File(std::path::PathBuf::from(r"playground.rs"))
    }

    fn after_analysis<'tcx>(
        &mut self, 
        _compiler: &rustc_interface::interface::Compiler, 
        queries: &'tcx rustc_interface::Queries<'tcx>
    ) -> rustc_driver::Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            for id in tcx.hir().items() {
                let item = tcx.hir().item(id);
                match item.kind {
                    rustc_hir::ItemKind::Fn(_, _, _) => {
                        println!("Function: {:?}", item.ident);
                    }
                    _ => {}
                }
            }
        });
        rustc_driver::Compilation::Continue
    }
}

fn main() {
    let args: Vec<String> = vec!["playground.rs".to_string()];
    let mut callbacks = rustc_driver::TimePassesCallbacks::default();
    let run_compiler = rustc_driver::RunCompiler::new(&args, &mut callbacks);
    let _ = run_compiler.run();
}
