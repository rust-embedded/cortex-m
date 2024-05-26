use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    println!("cargo:rustc-check-cfg=cfg(thumb)");
    if target.starts_with("thumbv") {
        println!("cargo:rustc-cfg=thumb");
    }
}
