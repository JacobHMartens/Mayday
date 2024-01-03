extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use rustc_hir::{intravisit::Visitor, FnSig, Item, ItemKind, Impl, Block, Unsafety, BlockCheckMode, UnsafeSource};
use rustc_middle::{hir::nested_filter, ty::TyCtxt};
use rustc_hir::intravisit::FnKind::{ItemFn, Method};

use crate::reporter::UnsafeReporter;

const UNSAFE_BLOCK: BlockCheckMode = BlockCheckMode::UnsafeBlock(UnsafeSource::UserProvided);

pub struct UnsafeCollector<'hir> {
    pub tcx: TyCtxt<'hir>,
    pub functions: Vec<FnSig<'hir>>,
    pub traits: Vec<Item<'hir>>,  // vector should only contain the 'Trait' variant of ItemKind
    pub impls: Vec<Item<'hir>>,
    pub blocks: Vec<Block<'hir>>,
    pub count_functions: usize,
    pub count_traits: usize,
    pub count_impls: usize,
    pub count_blocks: usize
}

impl<'hir> UnsafeReporter for UnsafeCollector<'hir> {
    fn report_unsafe_code(&self) {
        println!("Report:");
        println!("Unsafe functions: {:?} out of {:?} functions ({:.1}%)", self.functions.len(), self.count_functions, 100.0*self.functions.len() as f64 /self.count_functions as f64);
        println!("Unsafe Traits: {:?} out of {:?} traits ({:.1}%)", self.traits.len(), self.count_traits, 100.0*self.traits.len() as f64/self.count_traits as f64);
        println!("Unsafe Impls: {:?} out of {:?} trait implementations ({:.1}%)", self.impls.len(), self.count_impls, 100.0*self.impls.len() as f64/self.count_impls as f64);
        println!("Unsafe Blocks: {:?} out of {:?} blocks ({:.1}%)", self.blocks.len(), self.count_blocks, 100.0*self.blocks.len() as f64/self.count_blocks as f64);
    }
}

impl<'hir> Visitor<'hir> for UnsafeCollector<'hir> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.tcx.hir()
    }

    fn visit_foreign_item(&mut self, _: &'hir rustc_hir::ForeignItem<'hir>) {
        // Do nothing. We are not interested in unsafe code from external sources.
    }

    fn visit_item(&mut self, item: &'hir rustc_hir::Item<'hir>) {        
        // Collect unsafe traits and impls
        match item.kind {
            ItemKind::Trait(_, unsafety, _, _, _) => {
                if unsafety == Unsafety::Unsafe {
                    self.traits.push(*item)
                }
                self.count_traits += 1;
            }
            ItemKind::Impl(Impl { unsafety, .. }) => {
                if *unsafety == Unsafety::Unsafe {
                    self.impls.push(*item)
                }
                self.count_impls += 1;
            }
            _ => {}
        }

        rustc_hir::intravisit::walk_item(self, item);
    }
    
    fn visit_fn(&mut self, fn_kind: rustc_hir::intravisit::FnKind<'hir>, fn_decl: &'hir rustc_hir::FnDecl<'hir>, body_id: rustc_hir::BodyId, _: rustc_span::Span, def_id: rustc_hir::def_id::LocalDefId) {
        // Collect unsafe functions
        match fn_kind {
            ItemFn(ident, _, header) => {
                if header.unsafety == Unsafety::Unsafe {
                    self.functions.push(FnSig {header, decl: fn_decl, span: ident.span})
                }
                self.count_functions += 1;
            }
            Method(_, fn_sig) => {
                if fn_sig.header.unsafety == Unsafety::Unsafe {
                    self.functions.push(*fn_sig)
                }
                self.count_functions += 1;
            }
            _ => {}
        }
        rustc_hir::intravisit::walk_fn(self, fn_kind, fn_decl, body_id, def_id);
    }

    fn visit_block(&mut self, block: &'hir Block<'hir>) {
        // Collect unsafe blocks
        if block.rules == UNSAFE_BLOCK {
            self.blocks.push(*block);
        }
        self.count_blocks += 1;
        rustc_hir::intravisit::walk_block(self, block);
    }

    
}
