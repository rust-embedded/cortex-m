//! Process Stack Pointer Limit Register

use core::arch::asm;

/// Reads the CPU register
#[inline]
pub fn read() -> u32 {
    let r;
    unsafe { asm!("mrs {}, PSPLIM", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes `bits` to the CPU register
#[inline]
pub unsafe fn write(bits: u32) {
    asm!("msr PSPLIM, {}", in(reg) bits, options(nomem, nostack, preserves_flags));
}
