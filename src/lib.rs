type CStrPtr = usize;

extern "C" {
    pub fn register_device(scope: CStrPtr);
    pub fn device_error(err: CStrPtr);
}


#[no_mangle]
fn malloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}