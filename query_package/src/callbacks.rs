use rustc_driver::{Callbacks, Compilation};
use rustc_interface::{interface, Queries};
use rustc_hir::{ItemKind, ExprKind, Unsafety, Block, UnsafeSource};

pub struct CustomCallbacks;

impl Callbacks for CustomCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            let hir = tcx.hir();
            for id in hir.items() {
                let item = hir.item(id);
                match item.kind {
                    ItemKind::Fn(fn_sig, _, _) => {
                        if fn_sig.header.unsafety == Unsafety::Unsafe {
                            println!("Unsafe Function: {:?}", item.ident);
                        }
                    }
                    _ => {}
                }
            }
            for owner_id in hir.body_owners() {
                match hir.body(hir.body_owned_by(owner_id)).value.kind {
                    ExprKind::Block(block, _) => traverse_block(block),
                    _ => {}
                }
            }
        });
        Compilation::Continue
    }
}

fn traverse_block(block: &Block) {
    match block.expr {
        None => {}
        Some(expr) => {
            match expr.kind {
                ExprKind::Block(block, _) => traverse_block(block),
                _ => {}
            }
        }
    }
    if block.rules == rustc_hir::BlockCheckMode::UnsafeBlock(UnsafeSource::UserProvided) {
        println!("Unsafe block: {:?}", block.span);
    }
}