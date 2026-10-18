use arm_targets::Arch;
use std::{env, path::PathBuf};

fn main() {
    let target_info = arm_targets::process();

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let memory_x_file = match target_info.arch() {
        Some(Arch::Armv6M) => "memory_microbit.x",
        Some(Arch::Armv7M) | Some(Arch::Armv7EM) => "memory_lm3s6965evb.x",
        Some(Arch::Armv8MBase) | Some(Arch::Armv8MMain) => "memory_mps2_an505.x",
        _ => panic!("Unsupported target architecture: {:?}", target_info),
    };
    println!("cargo:rerun-if-changed={}", memory_x_file);
    let target_path = out.join("memory.x");

    // Copy memory.x from parent directory only if it doesn't exist locally
    std::fs::copy(memory_x_file, &target_path).expect("Failed to copy memory.x for target");

    // Tell linker to look where we just put the file
    println!("cargo:rustc-link-search={}", out.display());
}
