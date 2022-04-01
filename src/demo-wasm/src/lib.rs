use demo::add;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn show() {
    let result = add(1, 2);
    alert(&format!("1 + 2 = {}", result));
}
