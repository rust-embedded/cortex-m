//! Fault Mask Register

#[cfg(cortex_m)]
use core::arch::asm;

/// All exceptions are ...
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Faultmask {
    /// Active
    Active,
    /// Inactive, expect for NMI
    Inactive,
}

impl Faultmask {
    /// All exceptions are active
    #[inline]
    pub fn is_active(self) -> bool {
        self == Faultmask::Active
    }

    /// All exceptions, except for NMI, are inactive
    #[inline]
    pub fn is_inactive(self) -> bool {
        self == Faultmask::Inactive
    }
}

/// Reads the CPU register
#[cfg(cortex_m)]
#[inline]
pub fn read() -> Faultmask {
    let r: u32;
    unsafe { asm!("mrs {}, FAULTMASK", out(reg) r, options(nomem, nostack, preserves_flags)) };
    if r & (1 << 0) == (1 << 0) {
        Faultmask::Inactive
    } else {
        Faultmask::Active
    }
}
