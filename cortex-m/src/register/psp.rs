//! Process Stack Pointer

#[cfg(cortex_m)]
use core::arch::asm;

/// Reads the CPU register
#[cfg(cortex_m)]
#[inline]
pub fn read() -> u32 {
    let r;
    unsafe { asm!("mrs {}, PSP", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes `bits` to the CPU register
#[cfg(cortex_m)]
#[inline]
pub unsafe fn write(bits: u32) {
    // See comment on msp_w. Unlike MSP, there are legitimate use-cases for modifying PSP
    // if MSP is currently being used as the stack pointer.
    asm!("msr PSP, {}", in(reg) bits, options(nomem, nostack, preserves_flags));
}
