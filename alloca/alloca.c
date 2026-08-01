#include <stddef.h>
#include <stdint.h>

/**
 * `_MSC_VER` reference:
 * MSVC: https://learn.microsoft.com/en-us/cpp/overview/compiler-versions
 */
#ifdef _MSC_VER
#include <malloc.h>
#endif

/**
 * Callback function type for use with `c_with_alloca`.
 *
 * Rust declares this callback as `extern "C-unwind"` to allow unwinding through
 * FFI boundary.
 */
typedef void (*Callback)(uint8_t *ptr, void *data);

/**
 * Allocate `len` stack bytes and invoke `callback(ptr, data)` once.
 *
 * Pointer `ptr` is non-null and uniquely valid for `len` bytes during call,
 * `data` is passed through unchanged.
 */
void c_with_alloca(size_t len, Callback callback, void *data) {
    /**
     * `_MSC_VER` reference:
     * MSVC: https://learn.microsoft.com/en-us/cpp/overview/compiler-versions
     */
#ifdef _MSC_VER
    /**
     * Stack allocation with `_alloca`:
     * MSVC: https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/alloca
     */
    uint8_t *buffer = (uint8_t *) _alloca(len);
#else
    /**
     * C99 variable-length array (VLA) references:
     * GCC: https://gcc.gnu.org/onlinedocs/gcc/Variable-Length.html
     * Clang: https://clang.llvm.org/compatibility.html#variable-length-arrays
     */
    uint8_t buffer[len];
#endif

    callback(buffer, data);
}
