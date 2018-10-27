#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "../domUtils")]
extern {
//   fn appendNumberToBody(x: u32);
//   fn alert(x: u32);
  fn appendStringToBody(s: &str);
}

// Load a WebAssembly Function Written in Rust and Invoke it from JavaScript
// #[no_mangle]
// pub extern fn add_one(x: u32) -> u32 {
//   x + 1
// }

// Pass a JavaScript Function to WebAssembly and Invoke it from Rust
// #[no_mangle]
// pub extern fn run() {
//   unsafe {
//     appendNumberToBody(42);
//      alert(4);
//   }
// }

// Set up wasm-bindgen for easy Rust/JavaScript Interoperability
#[wasm_bindgen]
pub fn run() {
    appendStringToBody("Hello World");
}