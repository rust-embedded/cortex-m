extern crate chrono;
extern crate rustc_version;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use chrono::NaiveDate;

fn main() {
    let date: NaiveDate = rustc_version::version_meta()
        .unwrap()
        .commit_date
        .unwrap()
        .parse()
        .unwrap();

    if date > NaiveDate::from_ymd(2017, 12, 26) {
        println!("cargo:rustc-cfg=has_termination_lang")
    }

    let target = env::var("TARGET").unwrap();

    has_fpu(&target);
    is_armv6m(&target);

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("link.x"))
        .unwrap()
        .write_all(include_bytes!("link.x"))
        .unwrap();
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
