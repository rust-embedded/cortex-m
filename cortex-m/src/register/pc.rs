//! Program counter

#[cfg(cortex_m)]
use core::arch::asm;

/// Reads the CPU register
#[cfg(cortex_m)]
#[inline]
pub fn read() -> u32 {
    let r;
    unsafe { asm!("mov {}, pc", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes `bits` to the CPU register
#[cfg(cortex_m)]
#[inline]
pub unsafe fn write(bits: u32) {
    asm!("mov pc, {}", in(reg) bits, options(nomem, nostack, preserves_flags));
}
