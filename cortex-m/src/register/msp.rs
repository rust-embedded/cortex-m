//! Main Stack Pointer

#[cfg(cortex_m)]
use core::arch::asm;

/// Reads the CPU register
#[cfg(cortex_m)]
#[inline]
pub fn read() -> u32 {
    let r;
    unsafe { asm!("mrs {}, MSP", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes `bits` to the CPU register
#[cfg(cortex_m)]
#[inline]
#[deprecated = "calling this function invokes Undefined Behavior, consider asm::bootstrap as an alternative"]
pub unsafe fn write(bits: u32) {
    // Technically is writing to the stack pointer "not pushing any data to the stack"?
    // In any event, if we don't set `nostack` here, this method is useless as the new
    // stack value is immediately mutated by returning. Really this is just not a good
    // method and its use is marked as deprecated.
    asm!("msr MSP, {}", in(reg) bits, options(nomem, nostack, preserves_flags));
}

/// Reads the Non-Secure CPU register from Secure state.
///
/// Executing this function in Non-Secure state will return zeroes.
#[cfg(armv8m)]
#[inline]
pub fn read_ns() -> u32 {
    let r;
    unsafe { asm!("mrs {}, MSP_NS", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes `bits` to the Non-Secure CPU register from Secure state.
///
/// Executing this function in Non-Secure state will be ignored.
#[cfg(armv8m)]
#[inline]
pub unsafe fn write_ns(bits: u32) {
    asm!("msr MSP_NS, {}", in(reg) bits, options(nomem, nostack, preserves_flags));
}
