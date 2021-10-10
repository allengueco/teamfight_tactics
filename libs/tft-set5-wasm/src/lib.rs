use wasm_bindgen::prelude::*;
use lib_tft_set5 as tft;

#[wasm_bindgen]
pub fn hello() -> String {
    "hello world 222".into()
}

#[wasm_bindgen]
pub struct Units {
    units: Vec<tft::Unit<tft::SynergySet5>>,
}

#[wasm_bindgen]
impl Units {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            units: vec![
                tft::garen(),
                tft::gwen()
            ]
        }
    }
}