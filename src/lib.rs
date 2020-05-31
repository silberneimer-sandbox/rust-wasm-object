use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct SomeStructure {
    any_property: i32
}

#[wasm_bindgen]
impl SomeStructure {

    pub fn new() -> SomeStructure {
        SomeStructure {
            any_property: 0
        }
    }
}
