**THIS IS ONLY A ROUGH IDEA RIGHT NOW**

# wadi

Web Assembly Device Interface (`wadi`) is a complementary specification for `wasi` hosts that want to offer device drivers. Users may wish to extend/reduce the capabilities of the host environment at startup or runtime. This spec helps standadize the means by which `wasi` can communicate with web assembly device backed files.

# How it works

A host environment must first have a means by which device drive wasm modules are integrated into the running host. Consider a hypothetical device driver `cowbell.wasm` that plays a bell sound and a hypothetical wasmer environment.  We would first start by loading this driver.

```rust
libw::write_text("/_wasmer/kernel/driver","/home/richard/cowbell.wasm");
```

The device driver web assembly exposes a number of external facing functions that will be called by the host environment. Initially it will call `init()` on the device module.

The responsability of the device module will be to first register the scope of files that will be handled. Cow bell might only want a single file in your wasi host environment ('/dev/cowbell') but other devices, might want to handle whole heirarhies of directories (`/dev/usb/*`)

Once registered, the device will be usable like any other `wasi` file.

```rust
libw::write_text("/dev/cowbell","play");
```

`wadi` makes no restrictions on what additional host functions are exposed to web assembly module, allowing the device to do anything within the host's power it would like to allow.

# Example

```rust
use wadi::{register_scope,CString,cstr}
use js_ffi::*;

#[no_mangle]
pub fn init() {
    register_scope("/dev/cowbell");
}

#[no_mangle]
pub fn write(path: CString, data_ptr: usize, data_len: usize) {
    js!(window.alert).invoke_1("clonk!");
}

#[no_mangle]
pub fn name() -> usize {
    cstr("cowbell")
}
```

# wadi device interface

* init() - start the device and register scopes
* read(path,location, size, target) - read a certain number of bytes from a file path
* write(path,data,location,size) - write a certain number of bytes from a file
* query(path) - get information on one or many files controlled by the device
* malloc(size) - ask for some memory to be able to write data into
* free(ptr) - free data 
* name() - returns a name of the module

# wadi host interface
* register_device(file_scope, file_type)
* error(err)
