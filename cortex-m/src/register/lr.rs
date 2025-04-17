//! Link register

#[cfg(cortex_m)]
use core::arch::asm;

/// Reads the CPU register
///
/// Note that this function can't be used reliably: The value returned at least depends
/// on whether the compiler chooses to inline the function or not.
#[cfg(cortex_m)]
#[inline]
pub fn read() -> u32 {
    let r;
    unsafe { asm!("mov {}, lr", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

// No `write` function for the LR register, as it can't be used soundly.
