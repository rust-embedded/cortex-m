use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    has_fpu(&target);
    let is_armv6m = is_armv6m(&target);

    if target.starts_with("thumbv") {
        fs::copy(
            format!("bin/{}.a", target),
            out_dir.join("libcortex-m-rt.a"),
        ).unwrap();
        println!("cargo:rustc-link-lib=static=cortex-m-rt");
    }

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let link_x = include_bytes!("link.x.in");
    let mut f = if env::var_os("CARGO_FEATURE_DEVICE").is_some() {
        let mut f = File::create(out.join("link.x")).unwrap();

        writeln!(
            f,
            r#"
/* Provides weak aliases (cf. PROVIDED) for device specific interrupt handlers */
/* This will usually be provided by a device crate generated using svd2rust (see `device.x`) */
INCLUDE device.x"#
        ).unwrap();
        f.write_all(link_x).unwrap();
        f
    } else {
        let mut f = File::create(out.join("link.x")).unwrap();
        f.write_all(link_x).unwrap();
        f
    };

    let max_int_handlers = if is_armv6m { 32 } else { 240 };

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
    ).unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.x.in");
}

fn has_fpu(target: &str) {
    if target.ends_with("eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }
}

fn is_armv6m(target: &str) -> bool {
    if target.starts_with("thumbv6m-") {
        println!("cargo:rustc-cfg=armv6m");
        true
    } else {
        false
    }
}
