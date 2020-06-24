extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::fs;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn opFi(path: &str) {
    open(path);
}

fn open(path: &str)-> Result<(), Box<std::error::Error>> {
    let content = fs::read_to_string(path)?;
    alert(&format!("{}", content));
    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name));
}
