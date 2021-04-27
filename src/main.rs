extern crate pest;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate bitflags;

use std::fs::read_to_string;

use crate::build::Builder;
use crate::flags::CompilationFlags;

mod ast;
mod build;
mod flags;
mod types;
pub mod utils;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct LoopParser;

fn main() {
    let source = read_to_string("example.loop").expect("Cannot read example file");
    let ast = Builder::parse_and_purify(
        &source,
        Some(CompilationFlags::WHILE | CompilationFlags::LOOP | CompilationFlags::RETAIN_LNO),
    );
    println!("{}", ast.display(4, None))
}
