//! `cargo xtask` automation.
//!
//! Please refer to <https://github.com/matklad/cargo-xtask/> for an explanation of the concept.
//!
//! Also see the docs in `asm.rs`.

use std::collections::BTreeMap;
use std::env::current_dir;
use std::fs::{self, File};
use std::process::{Command, Stdio};

fn toolchain() -> String {
    fs::read_to_string("asm-toolchain")
        .unwrap()
        .trim()
        .to_string()
}

fn rustc() -> Command {
    let mut cmd = Command::new("rustc");
    cmd.arg(format!("+{}", toolchain()));
    cmd
}

fn assemble_really(target: &str, cfgs: &[&str], plugin_lto: bool) {
    let mut cmd = rustc();

    // Set the codegen target.
    cmd.arg("--target").arg(target);
    // Set all the `--cfg` directives for the target.
    cmd.args(cfgs.iter().map(|cfg| format!("--cfg={}", cfg)));

    // We want some level of debuginfo to allow unwinding through the functions.
    cmd.arg("-g");
    // We always optimize the assembly shims. There's not really any reason not to.
    cmd.arg("-O");

    // We use LTO on the archive to ensure the (unused) panic handler is removed, preventing
    // a linker error when the archives are linked into final crates with two panic handlers.
    cmd.arg("-Clto=yes");

    // rustc will usually add frame pointers by default to aid with debugging, but that is a high
    // overhead for the tiny assembly routines.
    cmd.arg("-Cforce-frame-pointers=no");

    // We don't want any system-specific paths to show up since we ship the result to other users.
    // Add `--remap-path-prefix $(pwd)=.`.
    let mut dir = current_dir().unwrap().as_os_str().to_os_string();
    dir.push("=.");
    cmd.arg("--remap-path-prefix").arg(dir);

    // We let rustc build a single object file, not a staticlib, since the latter pulls in loads of
    // code that will never be used (`compiler_builtins` and `core::fmt`, etc.). We build the static
    // archive by hand after compiling.
    cmd.arg("--emit=obj");

    if plugin_lto {
        // Make artifacts compatible with Linker-Plugin LTO (and incompatible with everything else).
        cmd.arg("-Clinker-plugin-lto");
    }

    let file_stub = if plugin_lto {
        format!("{}-lto", target)
    } else {
        target.to_string()
    };

    let obj_file = format!("bin/{}.o", file_stub);

    // Pass output and input file.
    cmd.arg("-o").arg(&obj_file);
    cmd.arg("asm/lib.rs");

    println!("{:?}", cmd);
    let status = cmd.status().unwrap();
    assert!(status.success());

    // Archive `target.o` -> `bin/target.a`.
    let mut builder = ar::Builder::new(File::create(format!("bin/{}.a", file_stub)).unwrap());

    // Use `append`, not `append_path`, to avoid adding any filesystem metadata (modification times,
    // etc.).
    let file = fs::read(&obj_file).unwrap();
    builder
        .append(
            &ar::Header::new(obj_file.as_bytes().to_vec(), file.len() as u64),
            &*file,
        )
        .unwrap();

    fs::remove_file(&obj_file).unwrap();
}

fn assemble(target: &str, cfgs: &[&str]) {
    assemble_really(target, cfgs, false);
    assemble_really(target, cfgs, true);
}

// `--target` -> `--cfg` list (mirrors what `build.rs` does).
static TARGETS: &[(&str, &[&str])] = &[
    ("thumbv6m-none-eabi", &[]),
    ("thumbv7m-none-eabi", &["armv7m"]),
    ("thumbv7em-none-eabi", &["armv7m", "armv7em"]),
    ("thumbv7em-none-eabihf", &["armv7m", "armv7em", "has_fpu"]),
    ("thumbv8m.base-none-eabi", &["armv8m", "armv8m_base"]),
    (
        "thumbv8m.main-none-eabi",
        &["armv7m", "armv8m", "armv8m_main"],
    ),
    (
        "thumbv8m.main-none-eabihf",
        &["armv7m", "armv8m", "armv8m_main", "has_fpu"],
    ),
];

pub fn install_targets(targets: &mut dyn Iterator<Item = &str>, toolchain: Option<&str>) {
    let mut rustup = Command::new("rustup");
    rustup.arg("target").arg("add").args(targets);

    if let Some(toolchain) = toolchain {
        rustup.arg("--toolchain").arg(toolchain);
    }

    let status = rustup.status().unwrap();
    assert!(status.success(), "rustup command failed: {:?}", rustup);
}

pub fn assemble_blobs() {
    let mut cmd = rustc();
    cmd.arg("-V");
    cmd.stdout(Stdio::null());
    let status = cmd.status().unwrap();
    let toolchain = toolchain();

    if !status.success() {
        println!(
            "asm toolchain {} does not seem to be installed. installing it now.",
            toolchain
        );

        let mut rustup = Command::new("rustup");
        let status = rustup.arg("install").arg(&toolchain).status().unwrap();
        assert!(status.success(), "rustup command failed: {:?}", rustup);
    }

    install_targets(
        &mut TARGETS.iter().map(|(target, _)| *target),
        Some(&*toolchain),
    );

    for (target, cfgs) in TARGETS {
        println!("building artifacts for {}", target);
        assemble(target, cfgs);
    }
}

pub fn check_blobs() {
    // Load each `.a` file in `bin` into memory.
    let mut files_before = BTreeMap::new();
    for entry in fs::read_dir("bin").unwrap() {
        let entry = entry.unwrap();
        // Only check -lto.a files since those have the same symbol order between Linux and macOS
        if entry.path().to_str().unwrap().ends_with("-lto.a") {
            files_before.insert(
                entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                fs::read(entry.path()).unwrap(),
            );
        }
    }

    assemble_blobs();

    let mut files_after = BTreeMap::new();
    for entry in fs::read_dir("bin").unwrap() {
        let entry = entry.unwrap();
        // Only check -lto.a files since those have the same symbol order between Linux and macOS
        if entry.path().to_str().unwrap().ends_with("-lto.a") {
            files_after.insert(
                entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                fs::read(entry.path()).unwrap(),
            );
        }
    }

    // Ensure they contain the same files.
    let before = files_before.keys().collect::<Vec<_>>();
    let after = files_after.keys().collect::<Vec<_>>();
    assert_eq!(before, after);

    for ((file, before), (_, after)) in files_before.iter().zip(files_after.iter()) {
        if before != after {
            panic!(
                "{} is not up-to-date, please run `cargo xtask assemble`",
                file
            );
        }
    }

    println!("Blobs identical.");
}
