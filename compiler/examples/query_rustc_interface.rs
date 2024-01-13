#![feature(rustc_private)]

/*
    Implementation using rustc interface
*/

extern crate rustc_interface;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_hash;
extern crate rustc_errors;
extern crate rustc_error_codes;
extern crate rustc_session;

use std::path::PathBuf;

use rustc_errors::registry;
use rustc_hir::ItemKind;
use rustc_session::config::{self, CheckCfg};

fn main() {

    let config = rustc_interface::Config {
        opts: Default::default(),
        input: config::Input::File(PathBuf::from(r"unsafe_example/src/main.rs")),
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
        hash_untracked_state: None
    };
    
    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
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
            })
        });
    });
}