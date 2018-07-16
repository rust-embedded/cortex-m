extern crate cc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.starts_with("thumb") && env::var_os("CARGO_FEATURE_INLINE_ASM").is_none() {
        // NOTE we need to place each routine in a separate assembly file or the linker won't be
        // able to discard the unused routines
        let mut build = cc::Build::new();
        build
            .file("asm/bkpt.s")
            .file("asm/control.s")
            .file("asm/cpsid.s")
            .file("asm/cpsie.s")
            .file("asm/delay.s")
            .file("asm/dmb.s")
            .file("asm/dsb.s")
            .file("asm/isb.s")
            .file("asm/msp_r.s")
            .file("asm/msp_w.s")
            .file("asm/nop.s")
            .file("asm/primask.s")
            .file("asm/psp_r.s")
            .file("asm/psp_w.s")
            .file("asm/sev.s")
            .file("asm/wfe.s")
            .file("asm/wfi.s");

        if target.starts_with("thumbv7m-") || target.starts_with("thumbv7em-") {
            build.file("asm/basepri_r.s");
            build.file("asm/faultmask.s");

            if env::var_os("CARGO_FEATURE_CM7_R0P1").is_some() {
                build.file("asm/basepri_max-cm7-r0p1.s");
                build.file("asm/basepri_w-cm7-r0p1.s");
            } else {
                build.file("asm/basepri_max.s");
                build.file("asm/basepri_w.s");
            }
        }

        build.compile("asm");
    }

    if target.starts_with("thumbv6m-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv6m");
    } else if target.starts_with("thumbv7m-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv7m");
    } else if target.starts_with("thumbv7em-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv7m");
        //println!("cargo:rustc-cfg=armv7em");
    }

    if target.ends_with("-eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }
}
