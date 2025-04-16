//! Link register

/// Reads the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[inline]
pub fn read() -> u32 {
    call_asm!(__lr_r() -> u32)
}

/// Writes `bits` to the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
///
/// # Safety
/// This function can't be used soundly.
#[inline]
#[deprecated = "This function can't be used soundly."]
pub unsafe fn write(bits: u32) {
    call_asm!(__lr_w(bits: u32));
}
