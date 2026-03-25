//! Main Stack Pointer

#[cfg(any(armv6m, armv7m, armv7em, armv8m))]
use core::arch::asm;

/// Reads the CPU register
#[inline]
#[cortex_m_macros::asm_cfg(any(armv6m, armv7m, armv7em, armv8m))]
pub fn read() -> u32 {
    let r;
    unsafe { asm!("mrs {}, MSP", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes `bits` to the CPU register
#[inline]
#[deprecated = "calling this function invokes Undefined Behavior, consider asm::bootstrap as an alternative"]
#[cortex_m_macros::asm_cfg(any(armv6m, armv7m, armv7em, armv8m))]
pub unsafe fn write(bits: u32) {
    // Technically is writing to the stack pointer "not pushing any data to the stack"?
    // In any event, if we don't set `nostack` here, this method is useless as the new
    // stack value is immediately mutated by returning. Really this is just not a good
    // method.
    unsafe { asm!("msr MSP, {}", in(reg) bits, options(nomem, nostack, preserves_flags)) };
}

/// Reads the Non-Secure CPU register from Secure state.
///
/// Executing this function in Non-Secure state will return zeroes.
#[inline]
#[cortex_m_macros::asm_cfg(armv8m)]
pub fn read_ns() -> u32 {
    let r;
    unsafe { asm!("mrs {}, MSP_NS", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes `bits` to the Non-Secure CPU register from Secure state.
///
/// Executing this function in Non-Secure state will be ignored.
#[inline]
#[cortex_m_macros::asm_cfg(armv8m)]
pub unsafe fn write_ns(bits: u32) {
    unsafe { asm!("msr MSP_NS, {}", in(reg) bits, options(nomem, nostack, preserves_flags)) };
}
