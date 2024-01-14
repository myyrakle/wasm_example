use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn add(lhs: Vec<u8>, rhs: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::with_capacity(lhs.len());

    for i in 0..lhs.len() {
        result.push(lhs[i] + rhs[i]);
    }

    return result;
}
