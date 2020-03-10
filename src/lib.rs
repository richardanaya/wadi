extern crate cstring;
pub use cstring::{cstr,CString};

extern "C" {
    pub fn register_scope(scope: CString);
    pub fn device_error(err: CString);
}

#[no_mangle]
fn malloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}