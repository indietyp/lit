#[macro_use]
extern crate bitflags;
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate derive_new;

use std::fs::read_to_string;

use crate::build::Builder;
use crate::flags::CompilationFlags;

mod ast;
mod build;
mod eval;
mod flags;
mod runtime;
mod types;
mod utils;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct LoopParser;

fn main() {
    let source = read_to_string("example2.loop").expect("Cannot read example file");
    println!(
        "{}",
        Builder::parse_and_compile(
            &source,
            Some(CompilationFlags::WHILE | CompilationFlags::RETAIN_LNO)
        )
        .display(4, None)
    );

    let mut runtime = Builder::all(
        &source,
        Some(CompilationFlags::WHILE | CompilationFlags::RETAIN_LNO),
        None,
    );

    // let running = true;
    while runtime.is_running() {
        let result = runtime.step();
        if let Some((lno, _)) = result {
            let lines = source.lines().collect::<Vec<&str>>();
            println!("{}: {}", lno, lines.get(lno - 1).unwrap_or(&"<Not Found>"));
        }
    }

    println!("{:?}", runtime.context())
}
