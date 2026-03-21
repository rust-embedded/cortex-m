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
    pub unsafe fn __msp_ns_r() -> u32 {
        let r;
        unsafe { asm!("mrs {}, MSP_NS", out(reg) r, options(nomem, nostack, preserves_flags)) };
        r
    }

    #[inline(always)]
    pub unsafe fn __msp_ns_w(val: u32) {
        unsafe { asm!("msr MSP_NS, {}", in(reg) val, options(nomem, nostack, preserves_flags)) };
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
