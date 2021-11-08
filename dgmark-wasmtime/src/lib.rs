use std::{
    alloc::{alloc, dealloc, Layout},
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

/// Struct holding a starting address and length.
#[repr(C)]
pub struct Array {
    pub offset: usize,
    pub len: usize,
}

/// Parses markdown and returns the list of translatable texts.
///
/// The caller is responsible for freeing the returned memory with `__dealloc`.
#[no_mangle]
pub fn texts(input: *const u8, len: usize) -> *const Array {
    let input =
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(input, len)) };

    let texts_vec = match dgmark::parse(input) {
        Ok(("", elements)) => elements
            .into_iter()
            .flat_map(|e| e.texts())
            .map(|t| {
                Box::into_raw(Box::new(Array {
                    offset: t.as_ptr() as usize,
                    len: t.len(),
                }))
            })
            .collect(),
        _ => vec![],
    };

    let texts = Box::new(Array {
        offset: texts_vec.as_ptr() as usize,
        len: texts_vec.len(),
    });
    let return_value = Box::into_raw(texts);

    // Force leaking of the Vec, otherwise it will be freed
    std::mem::forget(texts_vec);

    return_value
}
