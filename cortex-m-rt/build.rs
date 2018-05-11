extern crate cc;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();

    has_fpu(&target);
    is_armv6m(&target);

    if target.starts_with("thumbv") {
        cc::Build::new().file("asm.s").compile("asm");
    }

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let link_x = include_bytes!("link.x");
    if env::var_os("CARGO_FEATURE_DEVICE").is_some() {
        let mut f = File::create(out.join("link.x")).unwrap();

        writeln!(
            f,
            r#"
/* Provides weak aliases (cf. PROVIDED) for device specific interrupt handlers */
/* This will usually be provided by a device crate generated using svd2rust (see `device.x`) */
INCLUDE device.x"#
        ).unwrap();
        f.write_all(link_x).unwrap();
    } else {
        File::create(out.join("link.x"))
            .unwrap()
            .write_all(link_x)
            .unwrap();
    };
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.x");
}

fn has_fpu(target: &str) {
    if target.ends_with("eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }
}

fn is_armv6m(target: &str) {
    if target.starts_with("thumbv6m-") {
        println!("cargo:rustc-cfg=armv6m");
    }
}
