//! Priority mask register

#[cfg(cortex_m)]
use core::arch::asm;

/// Priority mask register
pub struct Primask(pub u32);

impl Primask {
    /// All exceptions with configurable priority are active
    #[inline]
    pub fn is_active(self) -> bool {
        !self.is_inactive()
    }

    /// All exceptions with configurable priority are inactive
    #[inline]
    pub fn is_inactive(self) -> bool {
        self.0 & (1 << 0) == (1 << 0)
    }
}

/// Reads the CPU register
#[cfg(cortex_m)]
#[inline]
pub fn read() -> Primask {
    let r: u32;
    unsafe { asm!("mrs {}, PRIMASK", out(reg) r, options(nomem, nostack, preserves_flags)) };
    Primask(r)
}

/// Writes the CPU register
#[cfg(cortex_m)]
#[inline]
pub fn write(r: u32) {
    unsafe { asm!("msr PRIMASK, {}", in(reg) r, options(nomem, nostack, preserves_flags)) };
}
