extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("hello, {}!", name));
}


#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("error");
    let document = window.document().expect("should have a document on window");
    // Manufacture the element we're gonna append
    let entry = document.get_element_by_id("app").expect("document should have a app");
    let val = document.create_element("p")?;
    let test = document.create_element("div")?;
    entry.append_child(&test)?;
    val.set_inner_html("Hello from Rust!");
    test.append_child(&val)?;
    Ok(())
}

