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

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_hir;
extern crate rustc_middle;

use rustc_middle::ty::TyCtxt;
use rustc_middle::query::Providers;

fn main() {
    let mut callbacks = rustc_driver::TimePassesCallbacks::default();
    let run_compiler = rustc_driver::RunCompiler::new(&std::env::args().collect::<Vec<String>>(), &mut callbacks);
    let item_type = tcx.type_of(def_id);
}




// Prusti interface
// LocalDefId is local to e.g. MIR, HIR etc. GlobalDefId is the same always. 