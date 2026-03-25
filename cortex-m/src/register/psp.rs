//! Process Stack Pointer

#[cfg(any(armv6m, armv7m, armv7em, armv8m))]
use core::arch::asm;

/// Reads the CPU register
#[inline]
#[cortex_m_macros::asm_cfg(any(armv6m, armv7m, armv7em, armv8m))]
pub fn read() -> u32 {
    let r;
    unsafe { asm!("mrs {}, PSP", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes `bits` to the CPU register
#[inline]
#[cortex_m_macros::asm_cfg(any(armv6m, armv7m, armv7em, armv8m))]
pub unsafe fn write(bits: u32) {
    // See comment on __msp_w. Unlike MSP, there are legitimate use-cases for modifying PSP
    // if MSP is currently being used as the stack pointer.
    unsafe { asm!("msr PSP, {}", in(reg) bits, options(nomem, nostack, preserves_flags)) };
}
