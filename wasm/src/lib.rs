use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn calc_add(a: u32, b: u32) -> u32 {
    return a + b;
}

#[wasm_bindgen]
pub fn calc_sub(a: u32, b: u32) -> u32 {
    return a - b;
}

#[wasm_bindgen]
pub fn calc_mult(a: u32, b: u32) -> u32 {
    return a * b;
}

#[wasm_bindgen]
pub fn calc_div(a: u32, b: u32) -> u32 {
    return a / b;
}
