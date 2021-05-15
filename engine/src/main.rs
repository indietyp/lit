#[macro_use]
extern crate bitflags;
extern crate pest;
#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate newtype_derive;

#[macro_use]
extern crate sum_type;

use std::fs::read_to_string;

use crate::build::Builder;
use crate::flags::CompileFlags;

mod ast;
mod build;
mod errors;
mod eval;
mod flags;
mod js;
mod parser;
mod runtime;
mod types;
mod utils;

#[cfg(test)]
mod tests;

#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "cli")]
use crate::cli::app;

#[cfg(feature = "cli")]
fn main() {
    app();
}

#[cfg(not(feature = "cli"))]
fn main() {
    println!("Enable the feature cli for Command Line support.")
}
