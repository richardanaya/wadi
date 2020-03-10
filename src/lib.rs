#![no_std]
extern crate alloc;
use alloc::vec::Vec;

extern crate cstring;
pub use cstring::{cstr, CString};

mod v1 {
    use super::*;
    extern "C" {
        pub fn register_scope(scope: CString);
        pub fn device_error(err: CString);
    }
}

pub fn register_scope(scope: &str) {
    unsafe {
        v1::register_scope(cstr(scope));
    }
}

pub fn device_error(scope: &str) {
    unsafe {
        v1::device_error(cstr(scope));
    }
}

#[no_mangle]
fn malloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}
