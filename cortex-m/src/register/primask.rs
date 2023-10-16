//! Priority mask register

#[cfg(cortex_m)]
use core::arch::asm;

/// All exceptions with configurable priority are ...
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primask {
    /// Active
    Active,
    /// Inactive
    Inactive,
}

impl Primask {
    /// All exceptions with configurable priority are active
    #[inline]
    pub fn is_active(self) -> bool {
        self == Primask::Active
    }

    /// All exceptions with configurable priority are inactive
    #[inline]
    pub fn is_inactive(self) -> bool {
        self == Primask::Inactive
    }
}

/// Reads the CPU register
#[cfg(cortex_m)]
#[inline]
pub fn read() -> Primask {
    let r: u32;
    unsafe { asm!("mrs {}, PRIMASK", out(reg) r, options(nomem, nostack, preserves_flags)) };
    if r & (1 << 0) == (1 << 0) {
        Primask::Inactive
    } else {
        Primask::Active
    }
}
