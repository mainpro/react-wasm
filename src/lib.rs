use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn big_computation() {
    alert("这个是一个超级耗时的复杂计算逻辑");
}

#[wasm_bindgen]
pub fn welcome(name: &str) {
   alert(&format!("Hi {} ，你正在和WebAssembly交互 ！", name));
}