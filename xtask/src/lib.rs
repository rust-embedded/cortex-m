//! `cargo xtask` automation.
//!
//! Please refer to <https://github.com/matklad/cargo-xtask/> for an explanation of the concept.
//!
//! Also see the docs in `asm.rs`.

use object::read::{Object as _, ObjectSection as _};
use object::write::{Object, Symbol, SymbolSection};
use object::{ObjectSymbol, SymbolFlags};
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

/// Patches an object file so that it doesn't contain a panic handler.
///
/// The panic handler defined in `asm/lib.rs` should never get linked to the final program.
/// Unfortunately, Rust uses the same symbol for all panic handlers, and doesn't really like it if
/// that ends up with multiple ones. It also demands that we define a panic handler for the inline
/// assembly shim, even though none of that code should ever be able to panic. The result of this is
/// that the supposedly unreachable panic handler does end up getting linked into the final program,
/// unless it is built with optimizations enabled.
///
/// To fix that, we put the never-to-be-used panic handler into its own section via
/// `#[link_section]`, and then use this function to delete that section.
fn trim_panic_handler(obj_file: &str) {
    let objdata = fs::read(&obj_file).unwrap();
    let obj = object::File::parse(&objdata).unwrap();

    let mut writer = Object::new(obj.format(), obj.architecture(), obj.endianness());
    for (sec_index, section) in obj.sections().enumerate() {
        assert_eq!(section.index().0, sec_index);

        let name = section.name().unwrap();
        if name.starts_with(".ARM")
            || name.starts_with(".rel.ARM")
            || name.contains("asm_panic_handler")
            || name == ".strtab"
            || name == ".symtab"
        {
            // We drop the ARM exception handling tables since they refer back to the panic handler
            // symbol. They aren't used either way. We also drop `.strtab` and `.symtab` since they
            // otherwise end up having the wrong section type. The object crate should rebuild any
            // index tables when writing the file.
            continue;
        }

        let segment = section
            .segment_name()
            .unwrap()
            .map(|s| s.as_bytes())
            .unwrap_or(&[]);
        let sec_id = writer.add_section(segment.to_vec(), name.as_bytes().to_vec(), section.kind());

        let align = if section.align() == 0 {
            // Not sure why but `section.align()` can return 0.
            1
        } else {
            section.align()
        };
        writer.append_section_data(sec_id, section.data().unwrap(), align);

        // Import all symbols from the section.
        for symbol in obj.symbols() {
            if symbol.section_index() == Some(section.index()) {
                writer.add_symbol(Symbol {
                    name: symbol.name().unwrap_or("").as_bytes().to_vec(),
                    value: symbol.address(),
                    size: symbol.size(),
                    kind: symbol.kind(),
                    scope: symbol.scope(),
                    weak: symbol.is_weak(),
                    section: match symbol.section() {
                        object::SymbolSection::Unknown => unimplemented!(),
                        object::SymbolSection::None => SymbolSection::None,
                        object::SymbolSection::Undefined => SymbolSection::Undefined,
                        object::SymbolSection::Absolute => SymbolSection::Absolute,
                        object::SymbolSection::Common => SymbolSection::Common,
                        object::SymbolSection::Section(_) => SymbolSection::Section(sec_id),
                    },
                    flags: match symbol.flags() {
                        SymbolFlags::None => SymbolFlags::None,
                        SymbolFlags::Elf { st_info, st_other } => {
                            SymbolFlags::Elf { st_info, st_other }
                        }
                        _ => unimplemented!(),
                    },
                });
            }
        }
    }

    let obj = writer.write().unwrap();
    fs::write(&obj_file, obj).unwrap();
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

    if !plugin_lto {
        // Post-process the object file.
        trim_panic_handler(&obj_file);
    }

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
        if entry.path().extension().unwrap() == "a" {
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
        if entry.path().extension().unwrap() == "a" {
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
