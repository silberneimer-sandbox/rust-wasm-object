use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
impl Point {

    pub fn zero() -> Point {
        Point {
            x: 0,
            y: 0
        }
    }
}