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
    for _ in 0..3 {
        cube = cube.perform(utils::dbg2!(Move::Face {
            face: FaceType::R,
            rotation_type: RotationType::Double,
            depth: 1,
        }));
        utils::log!("cube:\n{}", cube);
        cube = cube.perform(utils::dbg2!(Move::Face {
            face: FaceType::F,
            rotation_type: RotationType::Double,
            depth: 1,
        }));
        utils::log!("cube:\n{}", cube);
    }
    //utils::log!("cube:\n{}", cube);
}
