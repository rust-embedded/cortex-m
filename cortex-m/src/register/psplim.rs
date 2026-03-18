//! Process Stack Pointer Limit Register

/// Reads the CPU register
#[inline]
#[cortex_m_macros::asm_cfg(armv8m_main)]
pub fn read() -> u32 {
    unsafe { crate::asm::inner::__psplim_r() }
}

/// Writes `bits` to the CPU register
#[inline]
#[cortex_m_macros::asm_cfg(armv8m_main)]
pub unsafe fn write(bits: u32) {
    unsafe { crate::asm::inner::__psplim_w(bits) }
}
