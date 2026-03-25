//! Link register

#[cfg(cortex_m)]
use core::arch::asm;

/// Reads the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[inline]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub fn read() -> u32 {
    let r;
    unsafe { asm!("mov {}, lr", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes `bits` to the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
///
/// # Safety
/// This function can't be used soundly.
#[inline]
#[deprecated = "This function can't be used soundly."]
#[cortex_m_macros::asm_cfg(cortex_m)]
pub unsafe fn write(bits: u32) {
    unsafe { asm!("mov lr, {}", in(reg) bits, options(nomem, nostack, preserves_flags)) };
}
