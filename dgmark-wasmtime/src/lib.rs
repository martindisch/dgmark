use std::{
    alloc::{alloc, dealloc, Layout},
    slice, str,
};

/// Struct holding a starting address and length.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Array {
    pub offset: usize,
    pub len: usize,
}

/// Parses markdown and returns the list of translatable texts.
///
/// The caller is responsible for freeing the returned memory with
/// [`dealloc_texts`].
#[no_mangle]
pub fn texts(input: *const u8, len: usize) -> *const Array {
    let input =
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(input, len)) };

    let texts = dgmark::texts(input)
        .map(|texts| {
            texts
                .into_iter()
                .map(|t| {
                    Box::into_raw(Box::new(Array {
                        offset: t.as_ptr() as usize,
                        len: t.len(),
                    }))
                })
                .collect()
        })
        .unwrap_or_else(|_| vec![]);

    let texts_array = Box::into_raw(Box::new(Array {
        offset: texts.as_ptr() as usize,
        len: texts.len(),
    }));

    // Force leaking of the Vec, otherwise it will be dropped
    std::mem::forget(texts);

    texts_array
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

/// Frees all memory of the given [`Array`] of [`Array`]s.
#[no_mangle]
pub unsafe fn dealloc_texts(array: *const Array) {
    let array = *array;
    let align = std::mem::align_of::<Array>();

    for i in 0..array.len {
        let current_array = *((array.offset + i * align) as *const Array);
        __dealloc(current_array.offset as *mut u8, current_array.len);
    }

    __dealloc(array.offset as *mut u8, array.len);
}
