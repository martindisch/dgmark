use std::{
    alloc::{dealloc, Layout},
    ffi::{CStr, CString},
    os::raw::c_char,
};

/// Struct holding both the string array pointer and its size.
#[repr(C)]
pub struct Texts {
    pub texts: *const *mut c_char,
    pub len: i32,
}

/// Parses markdown and returns the list of translatable texts.
///
/// The caller is responsible for freeing the returned memory with
/// [`dealloc_texts`].
#[no_mangle]
pub extern "C" fn texts(input: *const c_char) -> Texts {
    let input = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input).to_str().unwrap()
    };

    let texts_vec = dgmark::texts(input)
        .map(|texts| {
            texts
                .into_iter()
                .map(|s| CString::new(s).unwrap().into_raw())
                .collect()
        })
        .unwrap_or_else(|_| vec![]);
    let texts_pointer = texts_vec.as_ptr();

    let texts = Texts {
        texts: texts_pointer,
        len: texts_vec.len() as i32,
    };

    // Force leaking of the Vec, otherwise it will be dropped
    std::mem::forget(texts_vec);

    texts
}

/// Frees all memory of the given [`Texts`].
#[no_mangle]
pub unsafe extern "C" fn dealloc_texts(texts: Texts) {
    // Retake ownership of all C strings to drop them
    for i in 0..texts.len {
        CString::from_raw(*texts.texts.offset(i as isize));
    }

    // Deallocate the Vec
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(texts.len as usize, align);
    dealloc(texts.texts as *mut u8, layout);
}
