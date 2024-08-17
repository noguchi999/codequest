extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    let msg = format!("Hello, {}!", name);
    alert(&msg);
}