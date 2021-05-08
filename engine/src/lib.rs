extern crate wasm_bindgen;

#[macro_use]
extern crate bitflags;

extern crate pest;

#[macro_use]
extern crate derive_new;

mod ast;
mod build;
mod errors;
mod eval;
mod flags;
mod js;
mod parser;
mod runtime;
mod tests;
mod types;
mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
