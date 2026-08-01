//! Unit tests covering allocation behavior and optional compiler stack
//! protections.

// Subprocess-based protection tests require standard library APIs.
#[cfg(any(feature = "stack-clash-protection", feature = "stack-protector"))]
extern crate std;

use core::mem::MaybeUninit;

// Keep test calls aligned with public crate paths.
mod alloca {
    pub use crate::*;
}

// Forwards callback result from basic byte allocation.
#[test]
fn test_with_alloca_simple() {
    let x = alloca::with_alloca(4096, |_| 42);
    assert_eq!(x, 42);
}

// Propagates panic through C unwind boundary.
#[cfg(not(all(target_arch = "arm", target_vendor = "unknown", target_os = "linux")))]
#[test]
#[should_panic]
fn test_with_alloca_panic() {
    alloca::with_alloca(4096, |_| panic!());
}

// Handles zero-length allocation without entering C code.
#[test]
fn test_with_alloca_zero_size() {
    let x = alloca::with_alloca(0, |memory| {
        assert!(memory.is_empty());
        42
    });
    assert_eq!(x, 42);
}

// Preserves writes across distant byte positions.
#[test]
fn test_with_alloca_complex() {
    let x = alloca::with_alloca(4096, |memory| {
        memory[0] = MaybeUninit::new(42);
        memory[1] = MaybeUninit::new(3);
        memory[3072] = MaybeUninit::new(4);
        unsafe {
            assert_eq!(memory[0].assume_init(), 42);
            assert_eq!(memory[1].assume_init(), 3);
            assert_eq!(memory[3072].assume_init(), 4);
            memory[0].assume_init() + memory[1].assume_init() + memory[3072].assume_init()
        }
    });
    assert_eq!(x, 42 + 3 + 4);
}

// Terminates subprocess when allocation exceeds thread stack.
#[cfg(all(
    feature = "stack-clash-protection",
    not(any(
        all(target_arch = "arm", target_vendor = "unknown", target_os = "linux"),
        target_family = "wasm",
    ))
))]
#[test]
fn test_with_alloca_stack_clash_protection() {
    use std::{env, process::Command, string::String, thread};

    const CHILD_ENV: &str = "ALLOCA_TEST_STACK_CLASH_PROTECTION_CHILD";
    const THREAD_STACK_SIZE: usize = 1024 * 1024;
    const ALLOCATION_SIZE: usize = 2 * THREAD_STACK_SIZE;

    match env::var_os(CHILD_ENV) {
        Some(_) => {
            thread::Builder::new()
                .stack_size(THREAD_STACK_SIZE)
                .spawn(|| alloca::with_alloca(ALLOCATION_SIZE, |_| {}))
                .expect("failed to spawn stack-clash child thread")
                .join()
                .expect("stack-clash child thread unexpectedly panicked");
        }
        _ => {
            let test_binary = env::current_exe().expect("failed to locate current test binary");
            let output = Command::new(test_binary)
                .args(["--exact", "tests::test_with_alloca_stack_clash_protection"])
                .env(CHILD_ENV, "1")
                .output()
                .expect("failed to run stack-clash-protection child test");

            assert!(
                !output.status.success(),
                "stack-clash protection did not terminate child process\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
        }
    }
}

// Detects stack frame corruption before C function returns.
#[cfg(all(
    feature = "stack-protector",
    not(any(
        all(
            any(target_arch = "arm", target_arch = "x86"),
            target_vendor = "unknown",
            target_os = "linux",
        ),
        all(target_vendor = "pc", target_os = "windows", target_env = "msvc"),
        target_family = "wasm",
    ))
))]
#[test]
fn test_with_alloca_stack_protector() {
    use std::{env, process::Command, string::String};

    const CHILD_ENV: &str = "ALLOCA_TEST_STACK_PROTECTOR_CHILD";

    match env::var_os(CHILD_ENV) {
        Some(_) => {
            let len = 128;
            alloca::with_alloca(len, |memory| {
                let ptr = memory.as_mut_ptr() as *mut u8;
                unsafe { ptr.add(len.saturating_add(8)).write_volatile(42) };
            });
        }
        _ => {
            let test_binary = env::current_exe().expect("failed to locate current test binary");
            let output = Command::new(test_binary)
                .args(["--exact", "tests::test_with_alloca_stack_protector"])
                .env(CHILD_ENV, "1")
                .output()
                .expect("failed to run stack-protector child test");

            assert!(
                !output.status.success(),
                "stack protector did not terminate child process\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
        }
    }
}

// Initializes every allocated byte with zero.
#[test]
fn test_with_alloca_zeroed() {
    let x = alloca::with_alloca_zeroed(4096, |memory| {
        assert!(memory.iter().all(|&x| x == 0));
        42
    });
    assert_eq!(x, 42);
}

// Provides correctly sized storage for u8.
#[test]
fn test_alloca_u8() {
    let x = alloca::alloca::<u8, _>(|slot| {
        slot.write(42);
        unsafe { slot.assume_init() }
    });
    assert_eq!(x, 42);
}

// Provides correctly aligned storage for u64.
#[test]
fn test_alloca_u64() {
    let x = alloca::alloca::<u64, _>(|slot| {
        slot.write(42);
        unsafe { slot.assume_init() }
    });
    assert_eq!(x, 42);
}

// Handles zero-sized types without C allocation.
#[test]
fn test_alloca_zst() {
    let x = alloca::alloca::<(), _>(|_| 42);
    assert_eq!(x, 42);
}

// Supports explicitly over-aligned types.
#[test]
fn test_alloca_aligned() {
    #[repr(align(16))]
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    struct Align16(u128);

    let x = alloca::alloca::<Align16, _>(|slot| {
        slot.write(Align16(42));
        unsafe { slot.assume_init() }
    });
    assert_eq!(x, Align16(42));
}
