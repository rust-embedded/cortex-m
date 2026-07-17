use std::{env, path::PathBuf};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MemorySelect {
    CortexM0,
    CortexM3,
}
fn main() {
    let target = std::env::var("TARGET").unwrap();

    println!("cargo:rustc-check-cfg=cfg(armv6m)");
    println!("cargo:rustc-check-cfg=cfg(armv7m)");
    println!("cargo:rustc-check-cfg=cfg(armv7em)");
    println!("cargo:rustc-check-cfg=cfg(armv8m)");
    println!("cargo:rustc-check-cfg=cfg(armv8m_base)");
    println!("cargo:rustc-check-cfg=cfg(armv8m_main)");

    let mut memory_select = None;
    if target.starts_with("thumbv6m-") {
        memory_select = Some(MemorySelect::CortexM0);
        println!("cargo:rustc-cfg=armv6m");
    } else if target.starts_with("thumbv7m-") {
        memory_select = Some(MemorySelect::CortexM3);
        println!("cargo:rustc-cfg=armv7m");
    } else if target.starts_with("thumbv7em-") {
        memory_select = Some(MemorySelect::CortexM3);
        println!("cargo:rustc-cfg=armv7m");
        println!("cargo:rustc-cfg=armv7em"); // (not currently used)
    } else if target.starts_with("thumbv8m.base") {
        println!("cargo:rustc-cfg=armv8m");
        println!("cargo:rustc-cfg=armv8m_base");
    } else if target.starts_with("thumbv8m.main") {
        println!("cargo:rustc-cfg=armv8m");
        println!("cargo:rustc-cfg=armv8m_main");
    }

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let memory_x_file = match memory_select {
        Some(MemorySelect::CortexM0) => {
            // Put `memory.x` in our output directory and ensure it's
            // on the linker search path.
            "memory_microbit.x"
        }
        Some(MemorySelect::CortexM3) => {
            // Put `memory.x` in our output directory and ensure it's
            // on the linker search path.
            "memory_m3.x"
        }
        // TODO: Copy memory.x if it exists?
        None => panic!("Unsupported target architecture: {}", target),
    };
    let target_path = out.join("memory.x");

    // Copy memory.x from parent directory only if it doesn't exist locally
    std::fs::copy(memory_x_file, &target_path)
        .unwrap_or_else(|e| panic!("Failed to copy memory.x: {}", e));

    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory_m3.x");
    println!("cargo:rerun-if-changed=memory_microbit.x");
}
