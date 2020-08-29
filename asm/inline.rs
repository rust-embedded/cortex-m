//! Inline assembly implementing the routines exposed in `cortex_m::asm`.
//!
//! If the `inline-asm` feature is enabled, these functions will be directly called by the
//! `cortex-m` wrappers. Otherwise, `cortex-m` links against them via prebuilt archives.
//!
//! All of these functions should be blanket-`unsafe`. `cortex-m` provides safe wrappers where
//! applicable.

#[inline(always)]
pub unsafe fn __bkpt() {
    asm!("bkpt");
}

#[inline(always)]
pub unsafe fn __control_r() -> u32 {
    let r;
    asm!("mrs {}, CONTROL", out(reg) r);
    r
}

#[inline(always)]
pub unsafe fn __control_w(w: u32) {
    asm!("msr CONTROL, {}", in(reg) w);
}

#[inline(always)]
pub unsafe fn __cpsid() {
    asm!("cpsid i");
}

#[inline(always)]
pub unsafe fn __cpsie() {
    asm!("cpsie i");
}

#[inline(always)]
pub unsafe fn __delay(cyc: u32) {
    asm!("
    1:
        nop
        subs {}, #1
        bne 1b
        // Branch to 1 instead of delay does not generate R_ARM_THM_JUMP8 relocation, which breaks
        // linking on the thumbv6m-none-eabi target
    ", in(reg) cyc);
}

// FIXME do we need compiler fences here or should we expect them in the caller?
#[inline(always)]
pub unsafe fn __dmb() {
    asm!("dmb 0xF");
}

#[inline(always)]
pub unsafe fn __dsb() {
    asm!("dsb 0xF");
}

#[inline(always)]
pub unsafe fn __isb() {
    asm!("isb 0xF");
}

#[inline(always)]
pub unsafe fn __msp_r() -> u32 {
    let r;
    asm!("mrs {}, MSP", out(reg) r);
    r
}

#[inline(always)]
pub unsafe fn __msp_w(val: u32) {
    asm!("msr MSP, {}", in(reg) val);
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
pub unsafe fn __apsr_r() -> u32 {
    let r;
    asm!("mrs {}, APSR", out(reg) r);
    r
}

#[inline(always)]
pub unsafe fn __nop() {
    // NOTE: This is a `pure` asm block, but applying that option allows the compiler to eliminate
    // the nop entirely (or to collapse multiple subsequent ones). Since the user probably wants N
    // nops when they call `nop` N times, let's not add that option.
    asm!("nop");
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
pub unsafe fn __pc_r() -> u32 {
    let r;
    asm!("mov {}, R15", out(reg) r);
    r
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
pub unsafe fn __pc_w(val: u32) {
    asm!("mov R15, {}", in(reg) val);
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
pub unsafe fn __lr_r() -> u32 {
    let r;
    asm!("mov {}, R14", out(reg) r);
    r
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
pub unsafe fn __lr_w(val: u32) {
    asm!("mov R14, {}", in(reg) val);
}

#[inline(always)]
pub unsafe fn __primask_r() -> u32 {
    let r;
    asm!("mrs {}, PRIMASK", out(reg) r);
    r
}

#[inline(always)]
pub unsafe fn __psp_r() -> u32 {
    let r;
    asm!("mrs {}, PSP", out(reg) r);
    r
}

#[inline(always)]
pub unsafe fn __psp_w(val: u32) {
    asm!("msr PSP, {}", in(reg) val);
}

#[inline(always)]
pub unsafe fn __sev() {
    asm!("sev");
}

#[inline(always)]
pub unsafe fn __udf() -> ! {
    asm!("udf #0", options(noreturn));
}

#[inline(always)]
pub unsafe fn __wfe() {
    asm!("wfe");
}

#[inline(always)]
pub unsafe fn __wfi() {
    asm!("wfi");
}

// v7m *AND* v8m.main, but *NOT* v8m.base
#[cfg(any(armv7m, armv8m_main))]
pub use self::v7m::*;
#[cfg(any(armv7m, armv8m_main))]
mod v7m {
    #[inline(always)]
    pub unsafe fn __basepri_max(val: u8) {
        asm!("msr BASEPRI_MAX, {}", in(reg) val);
    }

    #[inline(always)]
    pub unsafe fn __basepri_r() -> u8 {
        let r;
        asm!("mrs {}, BASEPRI", out(reg) r);
        r
    }

    #[inline(always)]
    pub unsafe fn __basepri_w(val: u8) {
        asm!("msr BASEPRI, {}", in(reg) val);
    }

    #[inline(always)]
    pub unsafe fn __faultmask_r() -> u32 {
        let r;
        asm!("mrs {}, FAULTMASK", out(reg) r);
        r
    }

    // FIXME: compiler_fences necessary?
    #[inline(always)]
    pub unsafe fn __enable_icache() {
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

    #[inline(always)]
    pub unsafe fn __enable_dcache() {
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
pub use self::v7em::*;
#[cfg(armv7em)]
mod v7em {
    #[inline(always)]
    pub unsafe fn __basepri_max_cm7_r0p1(val: u8) {
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

    #[inline(always)]
    pub unsafe fn __basepri_w_cm7_r0p1(val: u8) {
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

#[cfg(armv8m)]
pub use self::v8m::*;
/// Baseline and Mainline.
#[cfg(armv8m)]
mod v8m {
    #[inline(always)]
    pub unsafe fn __tt(mut target: u32) -> u32 {
        asm!("tt {target}, {target}", target = inout(reg) target);
        target
    }

    #[inline(always)]
    pub unsafe fn __ttt(mut target: u32) -> u32 {
        asm!("ttt {target}, {target}", target = inout(reg) target);
        target
    }

    #[inline(always)]
    pub unsafe fn __tta(mut target: u32) -> u32 {
        asm!("tta {target}, {target}", target = inout(reg) target);
        target
    }

    #[inline(always)]
    pub unsafe fn __ttat(mut target: u32) -> u32 {
        asm!("ttat {target}, {target}", target = inout(reg) target);
        target
    }
}

#[cfg(armv8m_main)]
pub use self::v8m_main::*;
/// Mainline only.
#[cfg(armv8m_main)]
mod v8m_main {
    #[inline(always)]
    pub unsafe fn __msplim_r() -> u32 {
        let r;
        asm!("mrs {}, MSPLIM", out(reg) r);
        r
    }

    #[inline(always)]
    pub unsafe fn __msplim_w(val: u32) {
        asm!("msr MSPLIM, {}", in(reg) val);
    }

    #[inline(always)]
    pub unsafe fn __psplim_r() -> u32 {
        let r;
        asm!("mrs {}, PSPLIM", out(reg) r);
        r
    }

    #[inline(always)]
    pub unsafe fn __psplim_w(val: u32) {
        asm!("msr PSPLIM, {}", in(reg) val);
    }
}

#[cfg(has_fpu)]
pub use self::fpu::*;
/// All targets with FPU.
#[cfg(has_fpu)]
mod fpu {
    #[inline(always)]
    pub unsafe fn __fpscr_r() -> u32 {
        let r;
        asm!("vmrs {}, fpscr", out(reg) r);
        r
    }

    #[inline(always)]
    pub unsafe fn __fpscr_w(val: u32) {
        asm!("vmsr fpscr, {}", in(reg) val);
    }
}
