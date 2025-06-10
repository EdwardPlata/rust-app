use wasm_bindgen::prelude::*;

// Import the `console.log` function from the browser
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro for easier console logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Export a `greet` function from Rust to JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) {
    console_log!("Hello, {}! Greetings from Rust and WebAssembly! 🦀", name);
}

// Export functions from shared crate
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    shared::add(a as u64, b as u64) as i32
}

// Export factorial function from shared crate
#[wasm_bindgen]
pub fn factorial(n: i32) -> i32 {
    shared::factorial(n as u64) as i32
}

// More complex example: working with arrays
#[wasm_bindgen]
pub fn sum_array(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

// Working with strings - use shared reverse_string
#[wasm_bindgen]
pub fn reverse_string(input: &str) -> String {
    shared::reverse_string(input)
}

// Example of using web APIs
#[wasm_bindgen]
pub fn get_window_dimensions() -> String {
    let window = web_sys::window().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap() as i32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as i32;
    format!("Window size: {}x{}", width, height)
}

// Called when the WASM module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("🦀 Rust WASM module loaded successfully!");
}
