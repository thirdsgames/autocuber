#![feature(maybe_uninit_uninit_array)]
#![feature(format_args_capture)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod cube;
mod group;
mod intuitive;
mod permute;
mod roux;
mod solve;
mod utils;

use wasm_bindgen::prelude::*;

use crate::cube::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Universe;

#[wasm_bindgen]
pub fn init() -> Universe {
    utils::set_panic_hook();
    Universe
}
