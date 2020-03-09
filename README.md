# wadi

Web Assembly Device Interface (`wadi`) is a complementary specification for `wasi` hosts that want to offer device drivers. Users may wish to extend/reduce the capabilities of the host environment at startup or runtime. This spec helps standadize the means by which `wasi` can communicate with device backed files.

# How it works

A host environment must first have a means by which device drive wasm modules are integrated into the running host. Consider a hypothetical device driver `cowbell.wasm` that plays a bell sound and a hypothetical wasmer environment.  We would first start by loading this driver.

```rust
libw::write_text("/_wasmer/kernel/driver","/home/richard/cowbell.wasm");
```

The device driver exposes a number of external facing functions that will be called by the host environment. Initially it will call `init()` on the device module.

The responsability of the device module will be to first register the scope of files that will be handled. Cow bell might only want a single file in your wasi host environment ('/dev/cowbell') but other devices, might want to handle whole heirarhies of directories (`/dev/usb/*`)

# wabi device interface

* init()
* read(path,size)
* write(path,data, size)
* seek(path,location)
* get_size(path)

# wabi host interface
* register_device(scope)
* error(err)