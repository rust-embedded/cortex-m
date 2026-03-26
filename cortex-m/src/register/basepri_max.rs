//! Base Priority Mask Register (conditional write)

#[cfg(any(armv7m, armv8m))]
use core::arch::asm;
use cortex_m_macros::asm_cfg;

/// Writes to BASEPRI *if*
///
/// - `basepri != 0` AND `basepri::read() == 0`, OR
/// - `basepri != 0` AND `basepri < basepri::read()`
///
/// **IMPORTANT** If you are using a Cortex-M7 device with revision r0p1 you MUST enable the
/// `cm7-r0p1` Cargo feature or this function WILL misbehave.
#[inline]
#[asm_cfg(any(armv7m, armv8m_main))]
pub fn write(basepri: u8) {
    #[cfg(not(feature = "cm7-r0p1"))]
    {
        unsafe {
            asm!("msr BASEPRI_MAX, {}", in(reg) basepri, options(nomem, nostack, preserves_flags))
        };
    }

    #[cfg(feature = "cm7-r0p1")]
    {
        unsafe {
            asm!(
                "mrs {1}, PRIMASK",
                "cpsid i",
                "tst.w {1}, #1",
                "msr BASEPRI_MAX, {0}",
                "it ne",
                "bxne lr",
                "cpsie i",
                in(reg) basepri,
                out(reg) _,
                options(nomem, nostack, preserves_flags),
            )
        };
    }
}
