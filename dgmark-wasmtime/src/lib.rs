use std::{
    alloc::{alloc, dealloc, Layout},
    ffi::CString,
    os::raw::c_char,
    slice, str,
};

/// Allocates some memory in this address space.
#[no_mangle]
pub unsafe fn __alloc(len: usize) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(len, align);
    alloc(layout)
}

/// Frees some memory in this address space.
#[no_mangle]
pub unsafe fn __dealloc(bytes: *mut u8, len: usize) {
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(len, align);
    dealloc(bytes, layout);
}

/// Struct holding both the string array pointer and its size.
#[repr(C)]
pub struct Texts {
    pub texts: *const *mut c_char,
    pub size: i32,
}

/// Parses markdown and returns the list of translatable texts.
///
/// The caller is responsible for freeing the returned memory with `__dealloc`.
#[no_mangle]
pub fn texts(input: *const u8, len: usize) -> *const Texts {
    let input =
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(input, len)) };

    let texts_vec = match dgmark::parse(input) {
        Ok(("", elements)) => elements
            .into_iter()
            .flat_map(|e| e.texts())
            .map(|s| CString::new(s).unwrap().into_raw())
            .collect(),
        _ => vec![],
    };
    let texts_pointer = texts_vec.as_ptr();

    let texts = Box::new(Texts {
        texts: texts_pointer,
        size: texts_vec.len() as i32,
    });
    let return_value = Box::into_raw(texts);

    // Force leaking of the Vec, otherwise it will be freed
    std::mem::forget(texts_vec);

    return_value
}
