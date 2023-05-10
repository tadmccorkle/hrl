# Hot Reloading in Rust

A simple library built around [libloading](https://docs.rs/libloading/latest) that uses [notify](https://docs.rs/crate/notify/latest) to reload functions and static symbols without requiring an application restart. Intended to aide in prototyping applications.

I made this because:

- I wanted the ability to hot-reload my Rust application's lib
- I thought it'd be a good exercise in Rust API design

There are other libraries and examples out there that use the same dependencies to do, effectively, the same thing:

- https://github.com/rksm/hot-lib-reloader-rs
- https://github.com/porglezomp-misc/live-reloading-rs
- https://github.com/emoon/dynamic_reload
- https://github.com/irh/rust-hot-reloading
- https://github.com/anirudhb/reloady
