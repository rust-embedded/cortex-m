//! Program counter

#[cfg(any(armv6m, armv7m, armv7em, armv8m))]
use core::arch::asm;

/// Reads the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[inline]
#[cortex_m_macros::asm_cfg(any(armv6m, armv7m, armv7em, armv8m))]
pub fn read() -> u32 {
    let r;
    unsafe { asm!("mov {}, pc", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes `bits` to the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[inline]
#[cortex_m_macros::asm_cfg(any(armv6m, armv7m, armv7em, armv8m))]
pub unsafe fn write(bits: u32) {
    unsafe { asm!("mov pc, {}", in(reg) bits, options(nomem, nostack, preserves_flags)) };
}
