extern crate pest;

#[macro_use]
extern crate pest_derive;

use crate::ast::PollutedASTNode::Macro;
use crate::ast::{ASTNode, PollutedASTNode};
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use std::fs::read_to_string;

mod ast;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LoopParser;

fn build_pure_ast_from_expression(pair: Pair<Rule>) -> ASTNode {
    // this code will be called if there i
    match pair.as_rule() {
        Rule::assignment => {
            let mut pair = pair.into_inner();

            ASTNode::Assignment {
                lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                rhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
            }
        }
        _ => panic!("You should not reach this, {}", _.as_str()),
    }
}

fn build_ast_from_expression(pair: Pair<Rule>) -> PollutedASTNode {
    match pair.as_rule() {
        Rule::macro_set_var_to_var => PollutedASTNode::Macro(Macro::AssignToVariable),
    }
}

fn parse(source: &str) -> Result<Vec<PollutedASTNode>, Error<Rule>> {
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
}
