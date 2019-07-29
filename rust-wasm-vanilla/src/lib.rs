extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

/**
 * 自身に定義する関数
 */
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

/**
 * 外に渡す関数
 */
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("hello, {}!", name));
}
