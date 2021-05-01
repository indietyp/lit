extern crate cfg_if;
extern crate wasm_bindgen;

#[macro_use]
extern crate bitflags;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use cfg_if::cfg_if;

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

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}
