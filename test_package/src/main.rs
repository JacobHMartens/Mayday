#![feature(rustc_private)]

/*
rustc_interface::run_compiler() [*1] is the main entrypoint to the compiler.
It takes a configuration for the compiler and a closure that takes a Compiler.
run_compiler creates a Compiler [*2] from the configuration and passes it to the closure.
Inside the closure, you can use the Compiler to drive queries to compile a crate and get the results.
This is what the rustc_driver does too. You can see a minimal example of how to use rustc_interface here [*3].

You can see what queries are currently available through the rustdocs for Compiler [*4].
You can see an example of how to use them by looking at the rustc_driver implementation, specifically the
rustc_driver::run_compiler [*5] function (not to be confused with rustc_interface::run_compiler [*6]).
The rustc_driver::run_compiler function takes a bunch of command-line args and some other configurations
and drives the compilation to completion.

rustc_driver::run_compiler also takes a Callbacks [*7], a trait that allows for custom compiler configuration,
as well as allowing some custom code run after different phases of the compilation.

[*1] https://doc.rust-lang.org/nightly/nightly-rustc/rustc_interface/interface/fn.run_compiler.html
[*2] https://doc.rust-lang.org/nightly/nightly-rustc/rustc_interface/interface/struct.Compiler.html
[*3] https://github.com/rust-lang/rustc-dev-guide/blob/master/examples/rustc-driver-example.rs
[*4] https://doc.rust-lang.org/nightly/nightly-rustc/rustc_interface/interface/struct.Compiler.html
[*5] https://doc.rust-lang.org/nightly/nightly-rustc/rustc_driver_impl/fn.run_compiler.html
[*6] https://doc.rust-lang.org/nightly/nightly-rustc/rustc_interface/interface/fn.run_compiler.html
[*7] https://doc.rust-lang.org/nightly/nightly-rustc/rustc_driver/trait.Callbacks.html
*/
/*
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use std::{path, process, str};

use rustc_errors::registry;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_session::config::{self, CheckCfg};
use rustc_span::source_map;

use rustc_middle::query::Providers;
use rustc_middle::ty::TyCtxt;

fn main() {
    // Using rustc driver
    // let mut callbacks = rustc_driver::TimePassesCallbacks::default();
    // let run_compiler = rustc_driver::RunCompiler::new(&std::env::args().collect::<Vec<String>>(), &mut callbacks);
    // let item_type = tcx.type_of(def_id);

}
*/

extern crate rustc_ast_pretty;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;

use std::path::PathBuf;

use rustc_ast_pretty::pprust::item_to_string;
use rustc_errors::registry;
use rustc_session::config::{self, CheckCfg};

fn main() {

    let config = rustc_interface::Config {
        opts: Default::default(),
        input: config::Input::File(PathBuf::from(r"playground.rs")),
        crate_cfg: rustc_hash::FxHashSet::default(),
        crate_check_cfg: CheckCfg::default(),
        output_dir: None,
        output_file: None,
        file_loader: None,
        locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES,
        lint_caps: rustc_hash::FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
        expanded_args: Vec::new(),
        ice_file: None,
    };
    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            let ast_krate = queries.parse().unwrap().get_mut().clone();
            for item in ast_krate.items {
                println!("{}", item_to_string(&item));
            }
            // Analyze the crate and inspect the types under the cursor.
            queries.global_ctxt().unwrap().enter(|tcx| {
                // Every compilation contains a single crate.
                let hir_krate = tcx.hir();
                // Iterate over the top-level items in the crate, looking for the main function.
                for id in hir_krate.items() {
                    let item = hir_krate.item(id);
                    // Use pattern-matching to find a specific node inside the main function.
                    if let rustc_hir::ItemKind::Fn(_, _, body_id) = item.kind {
                        let expr = &tcx.hir().body(body_id).value;
                        if let rustc_hir::ExprKind::Block(block, _) = expr.kind {
                            if let rustc_hir::StmtKind::Local(local) = block.stmts[0].kind {
                                if let Some(expr) = local.init {
                                    let hir_id = expr.hir_id; // hir_id identifies the string "Hello, world!"
                                    let def_id = item.hir_id().owner.def_id; // def_id identifies the main function
                                    let ty = tcx.typeck(def_id).node_type(hir_id);
                                    println!("{expr:#?}: {ty:?}");
                                }
                            }
                        }
                    }
                }
            })
        });
    });
}