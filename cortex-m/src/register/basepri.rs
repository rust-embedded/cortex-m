//! Base Priority Mask Register

#[cfg(any(armv7m, armv8m))]
use core::arch::asm;
#[cfg(any(armv7m, armv8m))]
use core::sync::atomic::{Ordering, compiler_fence};
use cortex_m_macros::asm_cfg;

/// Reads the CPU register
#[inline]
#[asm_cfg(any(armv7m, armv8m_main))]
pub fn read() -> u8 {
    let r;
    unsafe { asm!("mrs {}, BASEPRI", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes to the CPU register
///
/// Forms a read/write memory barrier for the compiler.
///
/// **IMPORTANT** If you are using a Cortex-M7 device with revision r0p1 you MUST enable the
/// `cm7-r0p1` Cargo feature or this function WILL misbehave.
#[inline]
#[asm_cfg(any(armv7m, armv8m_main))]
pub unsafe fn write(basepri: u8) {
    // Release before enabling interrupts; acquire after disabling them.
    // Work around missing asm synchronization effects: rust-lang/rust#144351.
    compiler_fence(Ordering::Release);

    #[cfg(not(feature = "cm7-r0p1"))]
    {
        unsafe {
            asm!(
                "msr BASEPRI, {}",
                "isb",
                in(reg) basepri,
                options(nostack, preserves_flags),
            )
        };
    }

    #[cfg(feature = "cm7-r0p1")]
    {
        unsafe {
            asm!(
                "mrs {1}, PRIMASK",
                "cpsid i",
                "tst.w {1}, #1",
                "msr BASEPRI, {0}",
                "isb",
                "it ne",
                "bxne lr",
                "cpsie i",
                in(reg) basepri,
                out(reg) _,
                options(nostack, preserves_flags),
            )
        };
    }
    compiler_fence(Ordering::Acquire);
}
