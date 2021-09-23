import * as wasm from "autocuber";

let universe = wasm.init();
console.log(universe);

wasm.greet();
universe.free();
