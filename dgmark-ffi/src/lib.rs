use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

// TODO: check if extern "C" is necessary
#[no_mangle]
pub extern "C" fn texts(input: *const c_char) -> *const *mut c_char {
    let input = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input).to_str().unwrap()
    };

    let texts = match dgmark::parse(input) {
        Ok(("", elements)) => elements
            .into_iter()
            .flat_map(|e| e.texts())
            .map(|s| CString::new(s).unwrap().into_raw())
            .collect(),
        _ => vec![],
    };

    texts.as_ptr()
}

// TODO: need extra function to deallocate our returned Vec
