use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

/// Struct holding both the string array pointer and its size.
#[repr(C)]
pub struct Texts {
    pub texts: *const *mut c_char,
    pub size: i32,
}

/// Parses markdown and returns the list of translatable texts.
#[no_mangle]
pub extern "C" fn texts(input: *const c_char) -> Texts {
    let input = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input).to_str().unwrap()
    };

    let texts_vec = match dgmark::parse(input) {
        Ok(("", elements)) => elements
            .into_iter()
            .flat_map(|e| e.texts())
            .map(|s| CString::new(s).unwrap().into_raw())
            .collect(),
        _ => vec![],
    };
    let texts_pointer = texts_vec.as_ptr();

    let texts = Texts {
        texts: texts_pointer,
        size: texts_vec.len() as i32,
    };

    // Force leaking of the Vec, otherwise it will be freed
    std::mem::forget(texts_vec);

    texts
}

// TODO: need extra function to deallocate our returned Vec
