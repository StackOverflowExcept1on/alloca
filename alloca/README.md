### What is alloca?

This library allows to allocate `len` bytes on stack and then pass uninitialized memory to Rust code.

### Contents of this directory

- [`alloca.c`](alloca.c) - library source code
- [`aarch64-apple-darwin/libcalloca.a`](aarch64-apple-darwin/libcalloca.a) - pre-built static library for `aarch64-apple-darwin` target
- [`x86_64-apple-darwin/libcalloca.a`](x86_64-apple-darwin/libcalloca.a) - pre-built static library for `x86_64-apple-darwin` target
- [`armv7-unknown-linux-gnueabi/libcalloca.a`](armv7-unknown-linux-gnueabi/libcalloca.a) - pre-built static library for `armv7-unknown-linux-gnueabi` target
- [`i686-unknown-linux-gnu/libcalloca.a`](i686-unknown-linux-gnu/libcalloca.a) - pre-built static library for `i686-unknown-linux-gnu` target
- [`aarch64-unknown-linux-gnu/libcalloca.a`](aarch64-unknown-linux-gnu/libcalloca.a) - pre-built static library for `aarch64-unknown-linux-gnu` target
- [`x86_64-unknown-linux-gnu/libcalloca.a`](x86_64-unknown-linux-gnu/libcalloca.a) - pre-built static library for `x86_64-unknown-linux-gnu` target
- [`armv7-unknown-linux-musleabi/libcalloca.a`](armv7-unknown-linux-musleabi/libcalloca.a) - pre-built static library for `armv7-unknown-linux-musleabi` target
- [`i686-unknown-linux-musl/libcalloca.a`](i686-unknown-linux-musl/libcalloca.a) - pre-built static library for `i686-unknown-linux-musl` target
- [`aarch64-unknown-linux-musl/libcalloca.a`](aarch64-unknown-linux-musl/libcalloca.a) - pre-built static library for `aarch64-unknown-linux-musl` target
- [`x86_64-unknown-linux-musl/libcalloca.a`](x86_64-unknown-linux-musl/libcalloca.a) - pre-built static library for `x86_64-unknown-linux-musl` target
- [`i686-pc-windows-msvc/calloca.lib`](i686-pc-windows-msvc/calloca.lib) - pre-built static library for `i686-pc-windows-msvc` target
- [`aarch64-pc-windows-msvc/calloca.lib`](aarch64-pc-windows-msvc/calloca.lib) - pre-built static library for `aarch64-pc-windows-msvc` target
- [`x86_64-pc-windows-msvc/calloca.lib`](x86_64-pc-windows-msvc/calloca.lib) - pre-built static library for `x86_64-pc-windows-msvc` target
- [`i686-pc-windows-gnu/libcalloca.a`](i686-pc-windows-gnu/libcalloca.a) - pre-built static library for `i686-pc-windows-gnu` target
- [`aarch64-pc-windows-gnullvm/libcalloca.a`](aarch64-pc-windows-gnullvm/libcalloca.a) - pre-built static library for `aarch64-pc-windows-gnullvm` target
- [`x86_64-pc-windows-gnu/libcalloca.a`](x86_64-pc-windows-gnu/libcalloca.a) - pre-built static library for `x86_64-pc-windows-gnu` target
- [`wasm32v1-none/libcalloca.a`](wasm32v1-none/libcalloca.a) - pre-built static library for `wasm32v1-none` target
- [`wasm32-unknown-unknown/libcalloca.a`](wasm32-unknown-unknown/libcalloca.a) - pre-built static library for `wasm32-unknown-unknown` target
- [`wasm32-unknown-emscripten/libcalloca.a`](wasm32-unknown-emscripten/libcalloca.a) - pre-built static library for `wasm32-unknown-emscripten` target
- [`wasm32-wasip1/libcalloca.a`](wasm32-wasip1/libcalloca.a) - pre-built static library for `wasm32-wasip1` target
- [`wasm32-wasip1-threads/libcalloca.a`](wasm32-wasip1-threads/libcalloca.a) - pre-built static library for `wasm32-wasip1-threads` target
- [`wasm32-wasip2/libcalloca.a`](wasm32-wasip2/libcalloca.a) - pre-built static library for `wasm32-wasip2` target

### Compiling vs pre-built library

Compilation should not happen in general case. We use pre-built library to not require C compiler.

However, if for some reason you want to compile C library at build time, use `compile-alloca` feature.
