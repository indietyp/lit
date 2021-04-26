extern crate pest;

#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate enum_display_derive;

use std::fs::read_to_string;
use std::str::FromStr;

use num_bigint::BigUint;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

use crate::ast::Node::Comparison;
use crate::ast::{ComparisonVerb, Control, Macro, Node, OperatorVerb, PollutedNode};
use crate::build::build_ast_from_expression;

mod ast;
mod build;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LoopParser;

fn parse(source: &str) -> Result<Vec<PollutedNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = LoopParser::parse(Rule::grammar, source)?;
    for pair in pairs {
        ast.push(build_ast_from_expression(pair));
    }

    Ok(ast)
}

fn main() {
    let unparsed = read_to_string("example.loop").expect("Cannot read example file");
    let polluted = parse(&unparsed).expect("Unsuccessful Parse");
    println!("{:#?}", polluted)
}
