use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.starts_with("thumbv6m-") {
        println!("cargo:rustc-cfg=armv6m");
    } else if target.starts_with("thumbv7m-") {
        println!("cargo:rustc-cfg=armv7m");
    } else if target.starts_with("thumbv7em-") {
        println!("cargo:rustc-cfg=armv7m");
        //println!("cargo:rustc-cfg=armv7em");
    }

    if target.ends_with("eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }
}
