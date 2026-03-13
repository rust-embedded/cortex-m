//! Link register

/// Reads the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[inline]
pub fn read() -> u32 {
    unsafe { crate::asm::inner::__lr_r() }
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
    unsafe { crate::asm::inner::__lr_w(bits) }
}
