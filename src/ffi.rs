use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[cfg(feature = "eac")]
use super::emoji::*;


#[cfg(feature = "eac")]
#[no_mangle]
pub extern fn ffi_emoji_generate(code: *const c_char) -> *const c_char {
    let code = unsafe {
        assert!(!code.is_null());
        CStr::from_ptr(code)
    };
    let code = code.to_str().unwrap();
    let result = emoji(code);

    // If we don't want to allocate more memory, we should save data in C's style.
    // Currently, we need to free the allocated memory later (call the corresponding free function)
    CString::new(*result.unwrap_or(&"")).unwrap().into_raw()
}

#[cfg(feature = "eac")]
#[no_mangle]
pub extern fn ffi_emoji_free(ptr: *mut c_char) {
    unsafe {
        if ptr.is_null() { return }
        CString::from_raw(ptr)
    };
}
