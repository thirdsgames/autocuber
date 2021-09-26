#![feature(maybe_uninit_uninit_array)]

mod cube;
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

#[wasm_bindgen]
pub fn greet() {
    // alert("Hello, autocuber!");
    let mut cube = Cube::<3>::new();
    let alg = "M2 U2 M2 U2".parse::<Algorithm>().unwrap();
    for mv in alg.moves {
        cube = cube.perform(utils::dbg2!(mv));
        utils::log!("cube:\n{}", cube);
    }
}

/// Generate some algorithm that we can perform on the cube.
#[wasm_bindgen]
pub fn gen_alg() -> AlgorithmConv {
    "R' U R' U' R' U' R' U R U R2"
        .parse::<Algorithm>()
        .unwrap()
        .into()
}
