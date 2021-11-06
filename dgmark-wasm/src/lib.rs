use js_sys::Array;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Parses markdown and returns the list of translatable texts.
#[wasm_bindgen]
pub fn texts(input: &str) -> Array {
    match dgmark::parse(input) {
        Ok(("", elements)) => {
            let array = Array::new();
            elements
                .into_iter()
                .flat_map(|e| e.texts())
                .map(JsValue::from_str)
                .for_each(|v| {
                    array.push(&v);
                });
            array
        }
        _ => Array::new(),
    }
}
