use wasm_bindgen::prelude::*;
use demo::add;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn show() {
    let result = add(1, 2);
    alert(&format!("1 + 2 = {}", result));
}
