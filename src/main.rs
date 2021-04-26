extern crate pest;

#[macro_use]
extern crate pest_derive;

mod ast;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LoopParser;

fn main() {
    println!("Hello, world!");
}
