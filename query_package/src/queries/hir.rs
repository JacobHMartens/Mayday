extern crate rustc_hir;
extern crate rustc_middle;

use rustc_hir::{Item, ItemKind, Unsafety, Block, BlockCheckMode, ExprKind, UnsafeSource, Expr};
use rustc_middle::hir::map::Map;

const UNSAFE_BLOCK: BlockCheckMode =  BlockCheckMode::UnsafeBlock(UnsafeSource::UserProvided);

struct Unsafeties<'hir> {
    unsafe_functions: Vec<Item<'hir>>,
}

// Returns all unsafe functions and traits from the provided HIR map
pub fn unsafe_functions_and_traits<'hir>(hir: Map<'hir>) -> Vec<Item<'hir>> {
    let mut unsafe_functions: Vec<Item<'hir>> = vec![];

    // Iterate over all HIR items to find function declarations
    for id in hir.items() {
        let item = hir.item(id);
        match item.kind {
            ItemKind::Fn(fn_sig, _, _) => {
                // Store function-item if it is declared as unsafe
                if fn_sig.header.unsafety == Unsafety::Unsafe {
                    unsafe_functions.push(*item);
                }
            }
            ItemKind::Trait(_, unsafety, _, _, _) => {
                if unsafety == Unsafety::Unsafe {

                }
            }
            _ => {}
        }
    }
    unsafe_functions
}

// Returns all unsafe blocks from the provided HIR map
pub fn unsafe_blocks<'hir>(hir: Map<'hir>) -> Vec<Block<'hir>> {
    let mut unsafe_blocks: Vec<Block<'hir>> = vec![];

    // Iterate over the bodies of all HIR owners
    for owner_id in hir.body_owners() {
        let body_expression: &Expr<'hir> = hir.body(hir.body_owned_by(owner_id)).value;
        // Search body for an unsafe block and store any found 
        if let Some(block) = try_find_unsafe_block(body_expression) {
            unsafe_blocks.push(*block);
        }
    }
    unsafe_blocks
}

// Returns some unsafe block or none after recursively searching the provided HIR expression
fn try_find_unsafe_block<'hir>(expr: &Expr<'hir>) -> Option<&'hir Block<'hir>> {
    match expr.kind {
        ExprKind::Block(block, _) if block.rules == UNSAFE_BLOCK => return Some(block),
        ExprKind::Block(block, _) => block.expr.and_then(try_find_unsafe_block),
        _ => return None
    }    
}
