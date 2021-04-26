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

fn purify(ast: &mut Vec<PollutedNode>) -> Node {
    let wrapped = PollutedNode::Control(Control::Terms(ast.clone()));

    wrapped.purify().flatten()
}

fn main() {
    let unparsed = read_to_string("example.loop").expect("Cannot read example file");
    let mut polluted = parse(&unparsed).expect("Unsuccessful Parse");
    let ast = purify(&mut polluted);
    println!("{:#?}", ast)
}
