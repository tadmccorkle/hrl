# Hot Reloading in Rust

A simple library built around [libloading](https://docs.rs/libloading/latest) that uses [notify](https://docs.rs/crate/notify/latest) to reload functions and static symbols without requiring an application restart. Intended to aide in prototyping applications. Created so I could hot-reload my Rust application's lib and practice Rust API design.

## Usage

Libraries can be hot reloaded automatically (i.e., when the file system changes) or manually (i.e., by explicit function call).

```rust
use hot_reload::library::auto::AutoHotReloadLibrary;

let lib = AutoHotReloadLibrary::load("path/to/lib").expect("failed to load library");

// Performing operations within the `symbol_op` function's closure ensures the symbol will
// always be valid (i.e., the library won't be reloaded before or during the symbol operation).

// call functions
let fn_result = lib
    .symbol_op::<unsafe extern "C" fn(u32) -> u32, _>("func", |func| unsafe { func(2) })
    .expect("function call failed");

// get and set static fields
lib.symbol_op::<*mut u32, _>("s_field", |s_field| unsafe { **s_field = 2 })
    .expect("failed to assign static field");
let s_field = lib
    .symbol_op::<*const u32, _>("s_field", |s_field| unsafe { **s_field })
    .expect("failed to access static field");


use hot_reload::library::manual::HotReloadLibrary;

let lib = HotReloadLibrary::load("path/to/lib").expect("failed to load library");

// call functions
let func = lib.symbol::<unsafe extern "C" fn(u32) -> u32>("func")
    .expect("failed to load function");
let fn_result = unsafe { func(2) };

// use static fields
let s_field = lib.symbol::<*mut u32>("s_field").expect("failed to load static field");
unsafe { **s_field = 2 };

// reload the library if its file has been changed
lib.reload().expect("failed to reload library");

// reload the library even if its file hasn't been changed
lib.force_reload().expect("failed to force reload library");
```

Macros can be used to make simple operations with the `AutoHotReloadLibrary`'s symbols a little less cumbersome.

```rust
use hot_reload::{
    auto_hrl_symbol_value, call_auto_hrl_symbol, library::auto::AutoHotReloadLibrary,
    set_auto_hrl_symbol_value,
};

let lib = AutoHotReloadLibrary::load("path/to/lib").expect("failed to load library");

let fn_result = call_auto_hrl_symbol!(lib, func(2): unsafe extern "C" fn(u32) -> u32)
    .expect("failed to call function");
set_auto_hrl_symbol_value!(lib, s_field: u32 = 2).expect("failed to assign static field");
let s_field = auto_hrl_symbol_value!(lib, s_field: u32).expect("failed to access static field");
```

Refer to the [examples](./examples) for additional usage and example project setup.

## Other Rust Hot Reload Libraries

There are other libraries and examples out there that use the same dependencies to do, effectively, the same thing as this library:

- https://github.com/rksm/hot-lib-reloader-rs
- https://github.com/porglezomp-misc/live-reloading-rs
- https://github.com/emoon/dynamic_reload
- https://github.com/irh/rust-hot-reloading
- https://github.com/anirudhb/reloady
