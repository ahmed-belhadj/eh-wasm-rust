extern {
  fn appendNumberToBody(x: u32);
  fn alert(x: u32);
}

// Load a WebAssembly Function Written in Rust and Invoke it from JavaScript
#[no_mangle]
pub extern fn add_one(x: u32) -> u32 {
  x + 1
}

// Pass a JavaScript Function to WebAssembly and Invoke it from Rust
#[no_mangle]
pub extern fn run() {
  unsafe {
    appendNumberToBody(42);
     alert(4);
  }
}