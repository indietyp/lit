#[macro_use]
extern crate enum_display_derive;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs::read_to_string;
use std::str::FromStr;

use num_bigint::BigUint;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

// use crate::ast::Node::Comparison;
// use crate::ast::{ComparisonVerb, Control, Macro, Node, OperatorVerb, PollutedNode};
use crate::build::Builder;

mod ast;
mod build;
mod utils;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LoopParser;

fn main() {
    let unparsed = read_to_string("example.loop").expect("Cannot read example file");
    let mut polluted = Builder::parse(&unparsed).expect("Unsuccessful Parse");
    let ast = purify(&mut polluted);
    println!("{}", ast.display(4, None))
}
