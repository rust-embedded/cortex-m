use far::{find, Render};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, ffi::OsStr};

const FLASH_REGION_ENV: &str = "CORTEX_M_RT_FLASH_REGION";
const RAM_REGION_ENV: &str = "CORTEX_M_RT_RAM_REGION";

#[derive(Render)]
struct LinkXReplacements {
    flash_region: String,
    ram_region: String,
}

fn main() {
    let mut target = env::var("TARGET").unwrap();

    // When using a custom target JSON, `$TARGET` contains the path to that JSON file. By
    // convention, these files are named after the actual target triple, eg.
    // `thumbv7m-customos-elf.json`, so we extract the file stem here to allow custom target specs.
    let path = Path::new(&target);
    if path.extension() == Some(OsStr::new("json")) {
        target = path
            .file_stem()
            .map_or(target.clone(), |stem| stem.to_str().unwrap().to_string());
    }

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let link_x = include_str!("link.x.in");

    // Replace regions in the linker script with the user's
    // specified region names, or defaults if not specified
    let tmpl = find::<_, LinkXReplacements>(link_x).unwrap();
    let mut replacements = LinkXReplacements {
        flash_region: "FLASH".to_owned(),
        ram_region: "RAM".to_owned(),
    };
    if let Ok(region) = env::var(FLASH_REGION_ENV) {
        println!("cargo:rerun-if-env-changed={}", FLASH_REGION_ENV);
        replacements.flash_region = region;
    };
    if let Ok(region) = env::var(RAM_REGION_ENV) {
        println!("cargo:rerun-if-env-changed={}", RAM_REGION_ENV);
        replacements.ram_region = region;
    };
    let link_x = tmpl.replace(&replacements);

    let mut f = if env::var_os("CARGO_FEATURE_DEVICE").is_some() {
        let mut f = File::create(out.join("link.x")).unwrap();

        f.write_all(link_x.as_bytes()).unwrap();

        // *IMPORTANT*: The weak aliases (i.e. `PROVIDED`) must come *after* `EXTERN(__INTERRUPTS)`.
        // Otherwise the linker will ignore user defined interrupts and always populate the table
        // with the weak aliases.
        writeln!(
            f,
            r#"
/* Provides weak aliases (cf. PROVIDED) for device specific interrupt handlers */
/* This will usually be provided by a device crate generated using svd2rust (see `device.x`) */
INCLUDE device.x"#
        )
        .unwrap();
        f
    } else {
        let mut f = File::create(out.join("link.x")).unwrap();
        f.write_all(link_x.as_bytes()).unwrap();
        f
    };

    println!("cargo:rustc-check-cfg=cfg(armv6m)");
    println!("cargo:rustc-check-cfg=cfg(armv7em)");
    println!("cargo:rustc-check-cfg=cfg(armv7m)");
    println!("cargo:rustc-check-cfg=cfg(armv8m)");
    println!("cargo:rustc-check-cfg=cfg(armv8m_base)");
    println!("cargo:rustc-check-cfg=cfg(armv8m_main)");
    println!("cargo:rustc-check-cfg=cfg(cortex_m)");
    println!("cargo:rustc-check-cfg=cfg(has_fpu)");

    let max_int_handlers = if target.starts_with("thumbv6m-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv6m");
        32
    } else if target.starts_with("thumbv7m-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv7m");
        240
    } else if target.starts_with("thumbv7em-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv7m");
        println!("cargo:rustc-cfg=armv7em");
        240
    } else if target.starts_with("thumbv8m.base") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv8m");
        println!("cargo:rustc-cfg=armv8m_base");
        240
    } else if target.starts_with("thumbv8m.main") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv8m");
        println!("cargo:rustc-cfg=armv8m_main");
        480
    } else {
        // Non ARM target. We assume you're just testing the syntax.
        // This value seems as good as any.
        240
    };

    if target.ends_with("-eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }

    // checking the size of the interrupts portion of the vector table is sub-architecture dependent
    writeln!(
        f,
        r#"
ASSERT(SIZEOF(.vector_table) <= 0x{:x}, "
There can't be more than {1} interrupt handlers. This may be a bug in
your device crate, or you may have registered more than {1} interrupt
handlers.");
"#,
        max_int_handlers * 4 + 0x40,
        max_int_handlers
    )
    .unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.x.in");
}
