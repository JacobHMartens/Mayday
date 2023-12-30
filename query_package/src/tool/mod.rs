use crate::compiler;


pub mod cli;

pub fn launch() {
    compiler::compile();
}