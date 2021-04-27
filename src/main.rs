extern crate pest;

#[macro_use]
extern crate pest_derive;

use std::fs::read_to_string;

use crate::build::Builder;

mod ast;
mod build;
mod flags;
pub mod utils;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct LoopParser;

fn main() {
    let source = read_to_string("example.loop").expect("Cannot read example file");
    let ast = Builder::parse_and_purify(&source);
    println!("{}", ast.display(4, None))
}
