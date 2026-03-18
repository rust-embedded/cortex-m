use core::arch::asm;
use core::sync::atomic::{Ordering, compiler_fence};

#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __control_r() -> u32 {
    let r;
    unsafe { asm!("mrs {}, CONTROL", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __control_w(w: u32) {
    // ISB is required after writing to CONTROL,
    // per ARM architectural requirements (see Application Note 321).
    unsafe {
        asm!(
            "msr CONTROL, {}",
            "isb",
            in(reg) w,
            options(nomem, nostack, preserves_flags),
        )
    };

    // Ensure memory accesses are not reordered around the CONTROL update.
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __cpsid() {
    unsafe { asm!("cpsid i", options(nomem, nostack, preserves_flags)) };

    // Ensure no subsequent memory accesses are reordered to before interrupts are disabled.
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn __cpsie() {
    // Ensure no preceeding memory accesses are reordered to after interrupts are enabled.
    compiler_fence(Ordering::SeqCst);

    unsafe { asm!("cpsie i", options(nomem, nostack, preserves_flags)) };
}

#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __delay(cyc: u32) {
    // The loop will normally take 3 to 4 CPU cycles per iteration, but superscalar cores
    // (eg. Cortex-M7) can potentially do it in 2, so we use that as the lower bound, since delaying
    // for more cycles is okay.
    // Add 1 to prevent an integer underflow which would cause a long freeze
    let real_cyc = 1 + cyc / 2;
    unsafe {
        asm!(
            // The `bne` on some cores (eg Cortex-M4) will take a different number of instructions
            // depending on the alignment of the branch target.  Set the alignment of the top of the
            // loop to prevent surprising timing changes when the alignment of the delay() changes.
            ".p2align 3",
            // Use local labels to avoid R_ARM_THM_JUMP8 relocations which fail on thumbv6m.
            "2:", // not 1 or 0 because of https://github.com/llvm/llvm-project/issues/99547
            "subs {}, #1", // subtract 1 from real_cyc
            "bne 2b",      // branch back to 2 if real_cyc is not zero
            inout(reg) real_cyc => _,
            options(nomem, nostack),
        )
    };
}

#[inline(always)]
pub unsafe fn __dmb() {
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("dmb", options(nostack, preserves_flags)) };
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn __dsb() {
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("dsb", options(nostack, preserves_flags)) };
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn __isb() {
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("isb", options(nostack, preserves_flags)) };
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __msp_r() -> u32 {
    let r;
    unsafe { asm!("mrs {}, MSP", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __msp_w(val: u32) {
    // Technically is writing to the stack pointer "not pushing any data to the stack"?
    // In any event, if we don't set `nostack` here, this method is useless as the new
    // stack value is immediately mutated by returning. Really this is just not a good
    // method and its higher-level use is marked as deprecated in cortex-m.
    unsafe { asm!("msr MSP, {}", in(reg) val, options(nomem, nostack, preserves_flags)) };
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __apsr_r() -> u32 {
    let r;
    unsafe { asm!("mrs {}, APSR", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __nop() {
    // NOTE: This is a `pure` asm block, but applying that option allows the compiler to eliminate
    // the nop entirely (or to collapse multiple subsequent ones). Since the user probably wants N
    // nops when they call `nop` N times, let's not add that option.
    unsafe { asm!("nop", options(nomem, nostack, preserves_flags)) };
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __pc_r() -> u32 {
    let r;
    unsafe { asm!("mov {}, pc", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __pc_w(val: u32) {
    unsafe { asm!("mov pc, {}", in(reg) val, options(nomem, nostack, preserves_flags)) };
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __lr_r() -> u32 {
    let r;
    unsafe { asm!("mov {}, lr", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __lr_w(val: u32) {
    unsafe { asm!("mov lr, {}", in(reg) val, options(nomem, nostack, preserves_flags)) };
}

#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __primask_r() -> u32 {
    let r;
    unsafe { asm!("mrs {}, PRIMASK", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __psp_r() -> u32 {
    let r;
    unsafe { asm!("mrs {}, PSP", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __psp_w(val: u32) {
    // See comment on __msp_w. Unlike MSP, there are legitimate use-cases for modifying PSP
    // if MSP is currently being used as the stack pointer.
    unsafe { asm!("msr PSP, {}", in(reg) val, options(nomem, nostack, preserves_flags)) };
}

#[inline(always)]
pub unsafe fn __sev() {
    unsafe { asm!("sev", options(nomem, nostack, preserves_flags)) };
}

#[inline(always)]
pub unsafe fn __udf() -> ! {
    unsafe { asm!("udf #0", options(noreturn, nomem, nostack, preserves_flags)) };
}

#[inline(always)]
pub unsafe fn __wfe() {
    unsafe { asm!("wfe", options(nomem, nostack, preserves_flags)) };
}

#[inline(always)]
pub unsafe fn __wfi() {
    unsafe { asm!("wfi", options(nomem, nostack, preserves_flags)) };
}

/// Set CONTROL.SPSEL to 0, write `msp` to MSP, branch to `rv`.
#[inline(always)]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn __bootstrap(msp: u32, rv: u32) -> ! {
    unsafe {
        asm!(
            "mrs {tmp}, CONTROL",
            "bics {tmp}, {spsel}",
            "msr CONTROL, {tmp}",
            "isb",
            "msr MSP, {msp}",
            "bx {rv}",
            // `out(reg) _` is not permitted in a `noreturn` asm! call,
            // so instead use `in(reg) 0` and don't restore it afterwards.
            tmp = in(reg) 0,
            spsel = in(reg) 2,
            msp = in(reg) msp,
            rv = in(reg) rv,
            options(noreturn, nomem, nostack),
        )
    };
}

#[cfg(any(not(cortex_m), armv7m, armv8m))]
pub(crate) use v7m::*;

#[cfg(any(not(cortex_m), armv7m, armv8m))]
pub(crate) mod v7m {
    use super::*;

    #[inline(always)]
    #[cortex_m_macros::asm_cfg(any(armv7m, armv8m_main))]
    pub unsafe fn __basepri_max(val: u8) {
        unsafe {
            asm!("msr BASEPRI_MAX, {}", in(reg) val, options(nomem, nostack, preserves_flags))
        };
    }

    #[inline(always)]
    #[cortex_m_macros::asm_cfg(any(armv7m, armv8m_main))]
    pub unsafe fn __basepri_r() -> u8 {
        let r;
        unsafe { asm!("mrs {}, BASEPRI", out(reg) r, options(nomem, nostack, preserves_flags)) };
        r
    }

    #[inline(always)]
    #[cortex_m_macros::asm_cfg(any(armv7m, armv8m_main))]
    pub unsafe fn __basepri_w(val: u8) {
        unsafe { asm!("msr BASEPRI, {}", in(reg) val, options(nomem, nostack, preserves_flags)) };
    }

    #[inline(always)]
    #[cortex_m_macros::asm_cfg(any(armv7m, armv8m_main))]
    pub unsafe fn __faultmask_r() -> u32 {
        let r;
        unsafe { asm!("mrs {}, FAULTMASK", out(reg) r, options(nomem, nostack, preserves_flags)) };
        r
    }

    // Should this be safe?
    #[inline(always)]
    pub unsafe fn __enable_icache() {
        unsafe {
            asm!(
                "ldr {0}, =0xE000ED14",         // CCR
                "mrs {2}, PRIMASK",             // save critical nesting info
                "cpsid i",                      // mask interrupts
                "ldr {1}, [{0}]",               // read CCR
                "orr.w {1}, {1}, #(1 << 17)",   // Set bit 17, IC
                "str {1}, [{0}]",               // write it back
                "dsb",                          // ensure store completes
                "isb",                          // synchronize pipeline
                "msr PRIMASK, {2}",             // unnest critical section
                out(reg) _,
                out(reg) _,
                out(reg) _,
                options(nostack),
            )
        };
        compiler_fence(Ordering::SeqCst);
    }

    // Should this be safe?
    #[inline(always)]
    pub unsafe fn __enable_dcache() {
        unsafe {
            asm!(
                "ldr {0}, =0xE000ED14",         // CCR
                "mrs {2}, PRIMASK",             // save critical nesting info
                "cpsid i",                      // mask interrupts
                "ldr {1}, [{0}]",               // read CCR
                "orr.w {1}, {1}, #(1 << 16)",   // Set bit 16, DC
                "str {1}, [{0}]",               // write it back
                "dsb",                          // ensure store completes
                "isb",                          // synchronize pipeline
                "msr PRIMASK, {2}",             // unnest critical section
                out(reg) _,
                out(reg) _,
                out(reg) _,
                options(nostack),
            )
        };
        compiler_fence(Ordering::SeqCst);
    }
}

#[cfg(feature = "cm7-r0p1")]
pub use self::v7em::*;

#[cfg(feature = "cm7-r0p1")]
mod v7em {
    use super::*;

    #[inline(always)]
    pub unsafe fn __basepri_max_cm7_r0p1(val: u8) {
        unsafe {
            asm!(
                "mrs {1}, PRIMASK",
                "cpsid i",
                "tst.w {1}, #1",
                "msr BASEPRI_MAX, {0}",
                "it ne",
                "bxne lr",
                "cpsie i",
                in(reg) val,
                out(reg) _,
                options(nomem, nostack, preserves_flags),
            )
        };
    }

    #[inline(always)]
    pub unsafe fn __basepri_w_cm7_r0p1(val: u8) {
        unsafe {
            asm!(
                "mrs {1}, PRIMASK",
                "cpsid i",
                "tst.w {1}, #1",
                "msr BASEPRI, {0}",
                "it ne",
                "bxne lr",
                "cpsie i",
                in(reg) val,
                out(reg) _,
                options(nomem, nostack, preserves_flags),
            )
        };
    }
}

#[cfg(armv8m)]
pub use self::v8m::*;
/// Baseline and Mainline.
#[cfg(armv8m)]
mod v8m {
    use super::*;

    #[inline(always)]
    pub unsafe fn __tt(mut target: u32) -> u32 {
        unsafe {
            asm!(
                "tt {target}, {target}",
                target = inout(reg) target,
                options(nomem, nostack, preserves_flags),
            )
        };
        target
    }

    #[inline(always)]
    pub unsafe fn __ttt(mut target: u32) -> u32 {
        unsafe {
            asm!(
                "ttt {target}, {target}",
                target = inout(reg) target,
                options(nomem, nostack, preserves_flags),
            )
        };
        target
    }

    #[inline(always)]
    pub unsafe fn __tta(mut target: u32) -> u32 {
        unsafe {
            asm!(
                "tta {target}, {target}",
                target = inout(reg) target,
                options(nomem, nostack, preserves_flags),
            )
        };
        target
    }

    #[inline(always)]
    pub unsafe fn __ttat(mut target: u32) -> u32 {
        unsafe {
            asm!(
                "ttat {target}, {target}",
                target = inout(reg) target,
                options(nomem, nostack, preserves_flags),
            )
        };
        target
    }

    #[inline(always)]
    pub unsafe fn __msp_ns_r() -> u32 {
        let r;
        unsafe { asm!("mrs {}, MSP_NS", out(reg) r, options(nomem, nostack, preserves_flags)) };
        r
    }

    #[inline(always)]
    pub unsafe fn __msp_ns_w(val: u32) {
        unsafe { asm!("msr MSP_NS, {}", in(reg) val, options(nomem, nostack, preserves_flags)) };
    }

    #[inline(always)]
    pub unsafe fn __bxns(val: u32) {
        unsafe { asm!("BXNS {}", in(reg) val, options(nomem, nostack, preserves_flags)) };
    }
}

#[cfg(armv8m_main)]
pub use self::v8m_main::*;
/// Mainline only.
#[cfg(armv8m_main)]
mod v8m_main {
    use super::*;

    #[inline(always)]
    pub unsafe fn __msplim_r() -> u32 {
        let r;
        unsafe { asm!("mrs {}, MSPLIM", out(reg) r, options(nomem, nostack, preserves_flags)) };
        r
    }

    #[inline(always)]
    pub unsafe fn __msplim_w(val: u32) {
        unsafe { asm!("msr MSPLIM, {}", in(reg) val, options(nomem, nostack, preserves_flags)) };
    }

    #[inline(always)]
    pub unsafe fn __psplim_r() -> u32 {
        let r;
        unsafe { asm!("mrs {}, PSPLIM", out(reg) r, options(nomem, nostack, preserves_flags)) };
        r
    }

    #[inline(always)]
    pub unsafe fn __psplim_w(val: u32) {
        unsafe { asm!("msr PSPLIM, {}", in(reg) val, options(nomem, nostack, preserves_flags)) };
    }
}

#[cfg(has_fpu)]
pub use self::fpu::*;
/// All targets with FPU.
#[cfg(has_fpu)]
mod fpu {
    use super::*;

    #[inline(always)]
    pub unsafe fn __fpscr_r() -> u32 {
        let r;
        unsafe { asm!("vmrs {}, fpscr", out(reg) r, options(nomem, nostack, preserves_flags)) };
        r
    }

    #[inline(always)]
    pub unsafe fn __fpscr_w(val: u32) {
        unsafe { asm!("vmsr fpscr, {}", in(reg) val, options(nomem, nostack)) };
    }
}

// /// We *must* define a panic handler here, even though nothing here should ever be able to panic.
// ///
// /// We prove that nothing will ever panic by calling a function that doesn't exist. If the panic
// /// handler gets linked in, this causes a linker error. We always build this file with optimizations
// /// enabled, but even without them the panic handler should never be linked in.
// #[panic_handler]
// #[unsafe(link_section = ".text.asm_panic_handler")]
// fn panic(_: &core::panic::PanicInfo) -> ! {
//     unsafe extern "C" {
//         #[link_name = "cortex-m internal error: panic handler not optimized out, please file an \
//         issue at https://github.com/rust-embedded/cortex-m"]
//         fn __cortex_m_should_not_panic() -> !;
//     }

//     unsafe {
//         __cortex_m_should_not_panic();
//     }
// }
