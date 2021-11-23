use js_sys::Array;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Parses markdown and returns the list of translatable texts.
#[wasm_bindgen]
pub fn texts(input: &str) -> JsValue {
    let result = dgmark::texts(input).unwrap_or_else(|_| vec![]);

    JsValue::from_serde(&result).unwrap()
}
