//! Assembly stubs for the `cortex-m` crate.
//!
//! We use this file to precompile some assembly stubs into the static libraries you can find in
//! `bin`. Apps using the `cortex-m` crate then link against those static libraries and don't need
//! to build this file themselves.
//!
//! Nowadays the assembly stubs are no longer actual assembly files, but actually just this Rust
//! file `asm.rs` that uses unstable inline assembly, coupled with the `xtask` tool to invoke rustc
//! and build the files.
//!
//! Precompiling this to a static lib allows users to call assembly routines from stable Rust, but
//! also perform [linker plugin LTO] with the precompiled artifacts to completely inline the
//! assembly routines into their code, which brings the "outline assembly" on par with "real" inline
//! assembly.
//!
//! For developers and contributors to `cortex-m`, this setup means that they don't have to install
//! any binutils, assembler, or C compiler to hack on the crate. All they need is to run `cargo
//! xtask assemble` to rebuild the archives from this file.
//!
//! Cool, right?
//!
//! # Rust version management
//!
//! Since inline assembly is still unstable, and we want to ensure that the created blobs are
//! up-to-date in CI, we have to pin the nightly version we use for this. The nightly toolchain is
//! stored in `asm-toolchain`.
//!
//! The `cargo xtask` automation will automatically install the `asm-toolchain` as well as all
//! Cortex-M targets needed to generate the blobs.
//!
//! [linker plugin LTO]: https://doc.rust-lang.org/stable/rustc/linker-plugin-lto.html

#![feature(asm)]
#![no_std]
#![crate_type = "staticlib"]
#![deny(warnings)]

#[no_mangle]
pub unsafe extern "C" fn __bkpt() {
    asm!("bkpt");
}

#[no_mangle]
pub unsafe extern "C" fn __control_r() -> u32 {
    let r;
    asm!("mrs {}, CONTROL", out(reg) r);
    r
}

#[no_mangle]
pub unsafe extern "C" fn __control_w(w: u32) {
    asm!("msr CONTROL, {}", in(reg) w);
}

#[no_mangle]
pub unsafe extern "C" fn __cpsid() {
    asm!("cpsid i");
}

#[no_mangle]
pub unsafe extern "C" fn __cpsie() {
    asm!("cpsie i");
}

#[no_mangle]
pub unsafe extern "C" fn __delay(cyc: u32) {
    asm!("
    1:
        nop
        subs {}, #1
        bne 1b
        // Branch to 1 instead of __delay does not generate R_ARM_THM_JUMP8 relocation, which breaks
        // linking on the thumbv6m-none-eabi target
    ", in(reg) cyc);
}

// FIXME do we need compiler fences here or should we expect them in the caller?
#[no_mangle]
pub unsafe extern "C" fn __dmb() {
    asm!("dmb 0xF");
}

#[no_mangle]
pub unsafe extern "C" fn __dsb() {
    asm!("dsb 0xF");
}

#[no_mangle]
pub unsafe extern "C" fn __isb() {
    asm!("isb 0xF");
}

#[no_mangle]
pub unsafe extern "C" fn __msp_r() -> u32 {
    let r;
    asm!("mrs {}, MSP", out(reg) r);
    r
}

#[no_mangle]
pub unsafe extern "C" fn __msp_w(val: u32) {
    asm!("msr MSP, {}", in(reg) val);
}

#[no_mangle]
pub unsafe extern "C" fn __nop() {
    // NOTE: This is a `pure` asm block, but applying that option allows the compiler to eliminate
    // the nop entirely (or to collapse multiple subsequent ones). Since the user probably wants N
    // nops when they call `nop` N times, let's not add that option.
    asm!("nop");
}

#[no_mangle]
pub unsafe extern "C" fn __primask() -> u32 {
    // FIXME: rename to __primask_r
    let r;
    asm!("mrs {}, PRIMASK", out(reg) r);
    r
}

#[no_mangle]
pub unsafe extern "C" fn __psp_r() -> u32 {
    let r;
    asm!("mrs {}, PSP", out(reg) r);
    r
}

#[no_mangle]
pub unsafe extern "C" fn __psp_w(val: u32) {
    asm!("msr PSP, {}", in(reg) val);
}

#[no_mangle]
pub unsafe extern "C" fn __sev() {
    asm!("sev");
}

#[no_mangle]
pub unsafe extern "C" fn __udf() {
    asm!("udf #0");
}

#[no_mangle]
pub unsafe extern "C" fn __wfe() {
    asm!("wfe");
}

#[no_mangle]
pub unsafe extern "C" fn __wfi() {
    asm!("wfi");
}

#[cfg(armv7m)]
pub mod v7m {
    #[no_mangle]
    pub unsafe extern "C" fn __basepri_max(val: u8) {
        asm!("msr BASEPRI_MAX, {}", in(reg) val);
    }

    #[no_mangle]
    pub unsafe extern "C" fn __basepri_r() -> u8 {
        let r;
        asm!("mrs {}, BASEPRI", out(reg) r);
        r
    }

    #[no_mangle]
    pub unsafe extern "C" fn __basepri_w(val: u8) {
        asm!("msr BASEPRI, {}", in(reg) val);
    }

    #[no_mangle]
    pub unsafe extern "C" fn __faultmask() -> u32 {
        let r;
        asm!("mrs {}, FAULTMASK", out(reg) r);
        r
    }

    // FIXME: compiler_fences necessary?
    #[no_mangle]
    pub unsafe extern "C" fn __enable_icache() {
        asm!(
            "
            ldr r0, =0xE000ED14       @ CCR
            mrs r2, PRIMASK           @ save critical nesting info
            cpsid i                   @ mask interrupts
            ldr r1, [r0]              @ read CCR
            orr.w r1, r1, #(1 << 17)  @ Set bit 17, IC
            str r1, [r0]              @ write it back
            dsb                       @ ensure store completes
            isb                       @ synchronize pipeline
            msr PRIMASK, r2           @ unnest critical section
            ",
            out("r0") _,
            out("r1") _,
            out("r2") _,
        );
    }

    #[no_mangle]
    pub unsafe extern "C" fn __enable_dcache() {
        asm!(
            "
            ldr r0, =0xE000ED14       @ CCR
            mrs r2, PRIMASK           @ save critical nesting info
            cpsid i                   @ mask interrupts
            ldr r1, [r0]              @ read CCR
            orr.w r1, r1, #(1 << 16)  @ Set bit 16, DC
            str r1, [r0]              @ write it back
            dsb                       @ ensure store completes
            isb                       @ synchronize pipeline
            msr PRIMASK, r2           @ unnest critical section
            ",
            out("r0") _,
            out("r1") _,
            out("r2") _,
        );
    }
}

#[cfg(armv7em)]
mod v7em {
    #[no_mangle]
    pub unsafe extern "C" fn __basepri_max_cm7_r0p1(val: u8) {
        asm!(
            "
            mrs r1, PRIMASK
            cpsid i
            tst.w r1, #1
            msr BASEPRI_MAX, {}
            it ne
            bxne lr
            cpsie i
            ",
            in(reg) val,
            out("r1") _,
        );
    }

    #[no_mangle]
    pub unsafe extern "C" fn __basepri_w_cm7_r0p1(val: u8) {
        asm!(
            "
            mrs r1, PRIMASK
            cpsid i
            tst.w r1, #1
            msr BASEPRI, {}
            it ne
            bxne lr
            cpsie i
            ",
            in(reg) val,
            out("r1") _,
        );
    }
}

/// Baseline and Mainline.
#[cfg(armv8m)]
pub mod v8m {
    #[no_mangle]
    pub unsafe extern "C" fn __tt(mut target: u32) -> u32 {
        asm!("tt {target}, {target}", target = inout(reg) target);
        target
    }

    #[no_mangle]
    pub unsafe extern "C" fn __ttt(mut target: u32) -> u32 {
        asm!("ttt {target}, {target}", target = inout(reg) target);
        target
    }

    #[no_mangle]
    pub unsafe extern "C" fn __tta(mut target: u32) -> u32 {
        asm!("tta {target}, {target}", target = inout(reg) target);
        target
    }

    #[no_mangle]
    pub unsafe extern "C" fn __ttat(mut target: u32) -> u32 {
        asm!("ttat {target}, {target}", target = inout(reg) target);
        target
    }
}

/// Mainline only.
#[cfg(armv8m_main)]
pub mod v8m_main {
    #[no_mangle]
    pub unsafe extern "C" fn __msplim_r() -> u32 {
        let r;
        asm!("mrs {}, MSPLIM", out(reg) r);
        r
    }

    #[no_mangle]
    pub unsafe extern "C" fn __msplim_w(val: u32) {
        asm!("msr MSPLIM, {}", in(reg) val);
    }

    #[no_mangle]
    pub unsafe extern "C" fn __psplim_r() -> u32 {
        let r;
        asm!("mrs {}, PSPLIM", out(reg) r);
        r
    }

    #[no_mangle]
    pub unsafe extern "C" fn __psplim_w(val: u32) {
        asm!("msr PSPLIM, {}", in(reg) val);
    }
}

/// All targets with FPU.
#[cfg(has_fpu)]
pub mod fpu {
    #[no_mangle]
    pub unsafe extern "C" fn __get_FPSCR() -> u32 {
        let r;
        asm!("vmrs {}, fpscr", out(reg) r);
        r
    }

    #[no_mangle]
    pub unsafe extern "C" fn __set_FPSCR(val: u32) {
        asm!("vmsr fpscr, {}", in(reg) val);
    }
}

/// We *must* define a panic handler here, even though nothing here should ever be able to panic.
///
/// We prove that nothing will ever panic by calling a function that doesn't exist. If the panic
/// handler gets linked in, this causes a linker error. We always build this file with optimizations
/// enabled, but even without them the panic handler should never be linked in.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    extern "C" {
        #[link_name = "cortex-m internal error: panic handler not optimized out, please file an \
        issue at https://github.com/rust-embedded/cortex-m"]
        fn __cortex_m_should_not_panic() -> !;
    }

    unsafe {
        __cortex_m_should_not_panic();
    }
}
