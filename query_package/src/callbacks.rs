use rustc_driver::{Callbacks, Compilation};
use rustc_interface::{interface, Queries};


use crate::queries::hir::{unsafe_functions, unsafe_blocks, unsafe_traits};

pub struct CustomCallbacks;

impl Callbacks for CustomCallbacks {

    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            let hir = tcx.hir();
            for item in unsafe_functions(hir) {
                println!("Unsafe Function: {:?}", item.ident);
            }
            for item in unsafe_traits(hir) {
                println!("Unsafe Trait: {:?}", item.ident);
            }
            
            for block in unsafe_blocks(hir) {
                println!("Unsafe block: {:?}", block.span);
            }
        });
        Compilation::Continue
    }
}
