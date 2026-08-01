### alloca

[![Build Status](https://github.com/StackOverflowExcept1on/alloca/workflows/CI/badge.svg)](https://github.com/StackOverflowExcept1on/alloca/actions/workflows/ci.yml)
[![Latest Version](https://img.shields.io/crates/v/alloca.svg)](https://crates.io/crates/alloca)
[![Documentation](https://docs.rs/alloca/badge.svg)](https://docs.rs/alloca)

Safe wrapper around [`alloca`](https://en.wikipedia.org/wiki/Stack-based_memory_allocation#System_interface).

Provides stack-allocated byte storage scoped to closure. Lifetimes prevent using slice after closure returns. If you convert memory to raw pointers and store them, safety is your responsibility.

### Example

```rust
use core::mem::MaybeUninit;

fn main() {
    // Allocate 128 bytes on stack
    alloca::with_alloca(128, |memory: &mut [MaybeUninit<u8>]| {
        assert_eq!(memory.len(), 128);
    });
}
```

### Supported targets

Pre-built static libraries are provided for:

- **macOS:**
  - `aarch64-apple-darwin`
  - `x86_64-apple-darwin`
- **Linux GNU:**
  - `armv7-unknown-linux-gnueabi`
  - `i686-unknown-linux-gnu`
  - `aarch64-unknown-linux-gnu`
  - `x86_64-unknown-linux-gnu`
- **Linux musl:**
  - `armv7-unknown-linux-musleabi`
  - `i686-unknown-linux-musl`
  - `aarch64-unknown-linux-musl`
  - `x86_64-unknown-linux-musl`
- **Windows:**
  - `i686-pc-windows-msvc`
  - `aarch64-pc-windows-msvc`
  - `x86_64-pc-windows-msvc`
  - `i686-pc-windows-gnu`
  - `aarch64-pc-windows-gnullvm`
  - `x86_64-pc-windows-gnu`
- **WebAssembly:**
  - `wasm32v1-none`
  - `wasm32-unknown-unknown`
  - `wasm32-unknown-emscripten`
  - `wasm32-wasip1`
  - `wasm32-wasip1-threads`
  - `wasm32-wasip2`

For other targets, enable `compile-alloca` feature and provide suitable C toolchain.

### Crate features

- **`compile-alloca`** — Enables compilation of C library `alloca.c` at build time.
- **`stack-clash-protection`** — Also enables `compile-alloca` and requires suitable C compiler. Enables stack-clash protection on non-MSVC targets with `-fstack-clash-protection`. Windows MSVC always has stack-clash protection enabled, regardless of this feature. Without this feature, non-MSVC targets use `-fno-stack-clash-protection`.
- **`stack-protector`** — Also enables `compile-alloca` and requires suitable C compiler. Enables stack protector support (compiler flag: `-fstack-protector-strong` or `/GS`). Without this feature, flag `-fno-stack-protector` or `/GS-` is used.
