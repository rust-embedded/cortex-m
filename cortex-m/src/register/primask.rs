//! Priority mask register

#[cfg(cortex_m)]
use core::arch::asm;
use core::sync::atomic::{compiler_fence, Ordering};

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

/// Reads the prioritizable interrupt mask
#[cfg(cortex_m)]
#[inline]
pub fn read() -> Primask {
    if read_raw() & (1 << 0) == (1 << 0) {
        Primask::Inactive
    } else {
        Primask::Active
    }
}

/// Reads the entire PRIMASK register
/// Note that bits [31:1] are reserved and UNK (Unknown)
#[cfg(cortex_m)]
#[inline]
pub fn read_raw() -> u32 {
    let r: u32;
    unsafe { asm!("mrs {}, PRIMASK", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes the entire PRIMASK register
/// Note that bits [31:1] are reserved and SBZP (Should-Be-Zero-or-Preserved)
#[cfg(cortex_m)]
#[inline]
pub fn write_raw(r: u32) {
    // Ensure no preceeding memory accesses are reordered to after interrupts are possibly enabled.
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("msr PRIMASK, {}", in(reg) r, options(nomem, nostack, preserves_flags)) };
    // Ensure no subsequent memory accesses are reordered to before interrupts are possibly disabled.
    compiler_fence(Ordering::SeqCst);
}
