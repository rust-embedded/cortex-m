fn main() {
    let target = std::env::var("TARGET").unwrap();

    println!("cargo:rustc-check-cfg=cfg(armv6m)");
    println!("cargo:rustc-check-cfg=cfg(armv7m)");
    println!("cargo:rustc-check-cfg=cfg(armv7em)");
    println!("cargo:rustc-check-cfg=cfg(armv8m)");
    println!("cargo:rustc-check-cfg=cfg(armv8m_base)");
    println!("cargo:rustc-check-cfg=cfg(armv8m_main)");

    if target.starts_with("thumbv6m-") {
        println!("cargo:rustc-cfg=armv6m");
    } else if target.starts_with("thumbv7m-") {
        println!("cargo:rustc-cfg=armv7m");
    } else if target.starts_with("thumbv7em-") {
        println!("cargo:rustc-cfg=armv7m");
        println!("cargo:rustc-cfg=armv7em"); // (not currently used)
    } else if target.starts_with("thumbv8m.base") {
        println!("cargo:rustc-cfg=armv8m");
        println!("cargo:rustc-cfg=armv8m_base");
    } else if target.starts_with("thumbv8m.main") {
        println!("cargo:rustc-cfg=armv8m");
        println!("cargo:rustc-cfg=armv8m_main");
    }
}
