#![allow(unused)]

pub mod units;
pub mod macros;

#[no_mangle]
unsafe extern "C" fn general_free_string(ptr: *mut std::ffi::c_char) {
    std::ffi::CString::from_raw(ptr);
}