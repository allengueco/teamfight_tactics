use lib_tft_set5 as tft;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> String {
    "hello world 222".into()
} 

#[wasm_bindgen]
pub struct Set5 {
    set: tft::Set5,
}