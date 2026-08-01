//! Safe wrapper around [`alloca`](https://en.wikipedia.org/wiki/Stack-based_memory_allocation#System_interface).
//!
//! Provides stack-allocated byte storage scoped to closure. Lifetimes prevent
//! using slice after closure returns. If you convert memory to raw pointers and
//! store them, safety is your responsibility.
//!
//! ### Example
//!
//! ```rust
//! use core::mem::MaybeUninit;
//!
//! fn main() {
//!     // Allocate 128 bytes on stack
//!     alloca::with_alloca(128, |memory: &mut [MaybeUninit<u8>]| {
//!         assert_eq!(memory.len(), 128);
//!     });
//! }
//! ```
//!
//! ### Supported targets
//!
//! Pre-built static libraries are provided for:
//!
//! - **macOS:**
//!   - `aarch64-apple-darwin`
//!   - `x86_64-apple-darwin`
//! - **Linux glibc:**
//!   - `armv7-unknown-linux-gnueabi`
//!   - `i686-unknown-linux-gnu`
//!   - `aarch64-unknown-linux-gnu`
//!   - `x86_64-unknown-linux-gnu`
//! - **Linux musl:**
//!   - `armv7-unknown-linux-musleabi`
//!   - `i686-unknown-linux-musl`
//!   - `aarch64-unknown-linux-musl`
//!   - `x86_64-unknown-linux-musl`
//! - **Windows:**
//!   - `i686-pc-windows-msvc`
//!   - `aarch64-pc-windows-msvc`
//!   - `x86_64-pc-windows-msvc`
//!   - `i686-pc-windows-gnu`
//!   - `aarch64-pc-windows-gnullvm`
//!   - `x86_64-pc-windows-gnu`
//! - **WebAssembly:**
//!   - `wasm32v1-none`
//!   - `wasm32-unknown-unknown`
//!   - `wasm32-unknown-emscripten`
//!   - `wasm32-wasip1`
//!   - `wasm32-wasip1-threads`
//!   - `wasm32-wasip2`
//!
//! For other targets, enable `compile-alloca` feature and provide suitable C
//! toolchain.
//!
//! ### Crate features
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
//!

#![no_std]

use core::{
    ffi::c_void,
    mem::{self, ManuallyDrop, MaybeUninit},
    slice,
};

/// Callback function type for use with [`c_with_alloca`].
///
/// Note: `extern "C-unwind"` is used to allow unwinding through FFI boundary.
type Callback = unsafe extern "C-unwind" fn(ptr: *mut MaybeUninit<u8>, data: *mut c_void);

unsafe extern "C-unwind" {
    /// Allocate `len` stack bytes and invoke `callback(ptr, data)` once.
    ///
    /// Pointer `ptr` is non-null and uniquely valid for `len` bytes during
    /// call, `data` is passed through unchanged.
    fn c_with_alloca(len: usize, callback: Callback, data: *mut c_void);
}

/// Returns trampoline specialized for `F` without naming closure type.
///
/// Returned [`Callback`] may be called once with valid allocation
/// pointer and owning pointer to `F`.
///
/// Based on Michael Bryan's closure-in-FFI pattern:
/// - https://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/#introducing-closures
#[inline(always)]
fn get_trampoline<F: FnOnce(*mut MaybeUninit<u8>)>(_closure: &F) -> Callback {
    trampoline::<F>
}

/// Invokes type-erased closure.
///
/// This is FFI trampoline that recovers stored `F` from `data` and forwards
/// allocated stack buffer pointer to it.
unsafe extern "C-unwind" fn trampoline<F: FnOnce(*mut MaybeUninit<u8>)>(
    ptr: *mut MaybeUninit<u8>,
    data: *mut c_void,
) {
    // SAFETY: Initialized: `let mut closure_data = ManuallyDrop::new(closure)`.
    let f = unsafe { ManuallyDrop::take(&mut *(data as *mut ManuallyDrop<F>)) };
    f(ptr);
}

/// Provides `len` uninitialized bytes of stack storage to `f`.
///
/// Slice is valid only for duration of `f`. Zero `len` is supported and
/// produces empty slice. Callback is invoked exactly once, and its return value
/// is forwarded.
///
/// # Stack usage
///
/// Allocation is not bounded by this function. Requesting more space than
/// current thread stack can provide may abort process or cause stack overflow.
/// Do not pass unbounded value derived from untrusted input.
///
/// # Compiler stack protections
///
/// Without `stack-clash-protection`, compiled C shim may omit stack probing.
/// Large allocations may skip stack guard pages, so caller must bound `len` and
/// accept platform-specific stack-allocation risks.
///
/// Without `stack-protector`, compiled C shim may omit stack canaries and
/// references to `__stack_chk_fail`. Safe writes through provided `&mut
/// [MaybeUninit<u8>]` remain bounds checked by Rust. This protection is
/// relevant when unsafe code writes beyond slice bounds through raw pointers or
/// foreign code accesses C interface directly. Rust provides no guarantees
/// after either contract violation.
pub fn with_alloca<R>(len: usize, f: impl FnOnce(&mut [MaybeUninit<u8>]) -> R) -> R {
    // C requires VLA to have size greater than zero.
    match len {
        0 => f(&mut []),
        len => {
            let mut ret = MaybeUninit::uninit();

            let closure = |ptr| {
                // SAFETY: C implementation guarantees that `ptr` is valid and was created with
                // `uint8_t buffer[len]`.
                let slice = unsafe { slice::from_raw_parts_mut(ptr, len) };
                ret.write(f(slice));
            };

            let trampoline = get_trampoline(&closure);
            let mut closure_data = ManuallyDrop::new(closure);

            unsafe {
                // SAFETY: `void *data` pattern is used to pass state, and trampoline closure is
                // used as state itself.
                c_with_alloca(len, trampoline, &mut closure_data as *mut _ as *mut c_void);
                ret.assume_init()
            }
        }
    }
}

/// Provides `len` zero-initialized bytes of stack storage to `f`.
///
/// This has same stack-usage considerations as [`with_alloca`]. Zero `len` is
/// supported.
pub fn with_alloca_zeroed<R>(len: usize, f: impl FnOnce(&mut [u8]) -> R) -> R {
    with_alloca(len, |memory| {
        memory.fill(MaybeUninit::zeroed());
        // SAFETY: Every element was initialized above, and every bit pattern is valid
        // for `u8`.
        f(unsafe { memory.assume_init_mut() })
    })
}

/// Provides stack storage for one potentially uninitialized `T` to `f`.
///
/// Slot is correctly aligned but not initialized. Zero-sized and over-aligned
/// types are supported. Callback's return value is forwarded.
pub fn alloca<T, R>(f: impl FnOnce(&mut MaybeUninit<T>) -> R) -> R {
    match mem::size_of::<T>() {
        0 => {
            let mut slot = MaybeUninit::uninit();
            f(&mut slot)
        }
        size => {
            let allocation_len = size.saturating_add(mem::align_of::<T>().saturating_sub(1));
            with_alloca(allocation_len, |memory| {
                // SAFETY: `memory` contains enough extra bytes to find aligned region large
                // enough for one `T`. `MaybeUninit<T>` imposes no initialization invariant.
                let (_, slot, _) = unsafe { memory.align_to_mut::<MaybeUninit<T>>() };
                // SAFETY: `slot` is guaranteed to have at least one element because `memory`
                // was allocated with enough space for one `T`.
                let slot = unsafe { slot.get_unchecked_mut(0) };
                f(slot)
            })
        }
    }
}

#[cfg(test)]
mod tests;
