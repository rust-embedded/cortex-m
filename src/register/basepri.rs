//! Base Priority Mask Register

#[cfg(cortex_m)]
use core::arch::asm;

/// Reads the CPU register
#[cfg(cortex_m)]
#[inline]
pub fn read() -> u8 {
    let r;
    unsafe { asm!("mrs {}, BASEPRI", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes to the CPU register
///
/// **IMPORTANT** If you are using a Cortex-M7 device with revision r0p1 you MUST enable the
/// `cm7-r0p1` Cargo feature or this function WILL misbehave.
#[cfg(cortex_m)]
#[inline]
pub unsafe fn write(basepri: u8) {
    #[cfg(feature = "cm7-r0p1")]
    {
        asm!(
            "mrs {1}, PRIMASK",
            "cpsid i",
            "tst.w {1}, #1",
            "msr BASEPRI, {0}",
            "it ne",
            "bxne lr",
            "cpsie i",
            in(reg) basepri,
            out(reg) _,
            options(nomem, nostack, preserves_flags),
        );
    }

    #[cfg(not(feature = "cm7-r0p1"))]
    {
        asm!("msr BASEPRI, {}", in(reg) basepri, options(nomem, nostack, preserves_flags));
    }
}
