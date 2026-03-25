//! Process Stack Pointer Limit Register

use core::arch::asm;
use cortex_m_macros::asm_cfg;

/// Reads the CPU register
#[inline]
#[asm_cfg(armv8m_main)]
pub fn read() -> u32 {
    let r;
    unsafe { asm!("mrs {}, PSPLIM", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes `bits` to the CPU register
#[inline]
#[asm_cfg(armv8m_main)]
pub unsafe fn write(bits: u32) {
    unsafe { asm!("msr PSPLIM, {}", in(reg) bits, options(nomem, nostack, preserves_flags)) };
}
