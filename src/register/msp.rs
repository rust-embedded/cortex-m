//! Main Stack Pointer

/// Reads the CPU register
#[inline]
pub fn read() -> u32 {
    call_asm!(__msp_r() -> u32)
}

/// Writes `bits` to the CPU register
#[inline]
pub unsafe fn write(bits: u32) {
    call_asm!(__msp_w(bits: u32));
}
