//! Main Stack Pointer Limit Register

/// Reads the CPU register
#[inline]
pub fn read() -> u32 {
    unsafe { crate::asm::inner::__msplim_r() }
}

/// Writes `bits` to the CPU register
#[inline]
pub unsafe fn write(bits: u32) {
    unsafe { crate::asm::inner::__msplim_w(bits) }
}
