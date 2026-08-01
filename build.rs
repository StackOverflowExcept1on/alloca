use std::{env, error::Error, path::PathBuf};

#[cfg(not(feature = "compile-alloca"))]
fn main() -> Result<(), Box<dyn Error>> {
    let alloca_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .join("alloca")
        .join(env::var("TARGET")?);

    println!("cargo::rustc-link-lib=static=calloca");
    println!("cargo::rustc-link-search=native={}", alloca_dir.display());

    Ok(())
}

#[cfg(feature = "compile-alloca")]
fn main() -> Result<(), Box<dyn Error>> {
    let alloca_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("alloca");

    let target_family = env::var("CARGO_CFG_TARGET_FAMILY")?;
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH")?;
    let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR")?;
    let target_env = env::var("CARGO_CFG_TARGET_ENV")?;
    let target_abi = env::var("CARGO_CFG_TARGET_ABI")?;

    let mut builder = cc::Build::new();

    if builder.get_compiler().is_like_msvc() {
        // Produce deterministic static library archives.
        builder.ar_flag("/Brepro");

        // On MSVC feature `stack-clash-protection` is always enabled.

        #[cfg(feature = "stack-protector")]
        builder.flag("/GS");

        #[cfg(not(feature = "stack-protector"))]
        builder.flag("/GS-");
    } else {
        let is_apple = target_vendor == "apple";
        let is_arm = target_arch == "arm";
        let is_gnullvm = target_env == "gnu" && target_abi == "llvm";
        let is_wasm = target_family.split(',').any(|family| family == "wasm");

        let stack_clash_protection_unsupported = is_apple || is_arm || is_gnullvm || is_wasm;

        if !stack_clash_protection_unsupported {
            #[cfg(feature = "stack-clash-protection")]
            builder.flag_if_supported("-fstack-clash-protection");

            #[cfg(not(feature = "stack-clash-protection"))]
            builder.flag_if_supported("-fno-stack-clash-protection");
        }

        #[cfg(feature = "stack-protector")]
        builder.flag_if_supported("-fstack-protector-strong");

        #[cfg(not(feature = "stack-protector"))]
        builder.flag_if_supported("-fno-stack-protector");
    }

    builder
        .file(alloca_dir.join("alloca.c"))
        .opt_level(2)
        .compile("calloca");

    Ok(())
}
