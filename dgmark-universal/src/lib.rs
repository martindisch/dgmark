use std::{
    alloc::{alloc, dealloc, Layout},
    slice, str,
};

/// Struct holding a starting address and length.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ByteArray {
    pub ptr: *const u8,
    pub len: usize,
}

/// Parses markdown and returns the list of translatable texts.
///
/// The caller is responsible for using [`__dealloc`] to free
/// - The allocated memory of the input string
/// - The returned byte array
/// - The returned byte array descriptor
#[no_mangle]
pub fn texts(ptr: *const u8, len: usize) -> *const ByteArray {
    let input =
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr, len)) };

    let texts = dgmark::texts(input).unwrap_or_else(|_| vec![]);
    let serialized_texts = serde_json::to_string(&texts).unwrap();

    let texts_descriptor = Box::into_raw(Box::new(ByteArray {
        ptr: serialized_texts.as_ptr(),
        len: serialized_texts.len(),
    }));

    // Force leaking of the serialized string, otherwise it will be dropped
    std::mem::forget(serialized_texts);

    texts_descriptor
}

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
