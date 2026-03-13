//! Program counter

/// Reads the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[inline]
pub fn read() -> u32 {
    unsafe { crate::asm::inner::__pc_r() }
}

/// Writes `bits` to the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[inline]
pub unsafe fn write(bits: u32) {
    unsafe { crate::asm::inner::__pc_w(bits) }
}
