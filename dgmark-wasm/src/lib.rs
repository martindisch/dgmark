use js_sys::Array;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Parses markdown and returns the list of translatable texts.
#[wasm_bindgen]
pub fn texts(input: &str) -> Result<Array, JsValue> {
    match dgmark::texts(input) {
        Ok(texts) => {
            let array = Array::new();
            texts.into_iter().map(JsValue::from_str).for_each(|v| {
                array.push(&v);
            });
            Ok(array)
        }
        Err(e) => Err(JsValue::from_str(e)),
    }
}
