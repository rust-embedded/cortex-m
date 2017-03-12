//! Fault Mask Register

/// All exceptions are ...
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Faultmask {
    /// Active
    Active,
    /// Inactive, expect for NMI
    Inactive,
}

impl Faultmask {
    /// All exceptions are active
    pub fn is_active(&self) -> bool {
        *self == Faultmask::Active
    }

    /// All exceptions, except for NMI, are inactive
    pub fn is_inactive(&self) -> bool {
        *self == Faultmask::Inactive
    }
}

/// Reads the CPU register
#[inline(always)]
pub fn read() -> Faultmask {
    let r: u32;
    unsafe {
        asm!("mrs $0, FAULTMASK"
             : "=r"(r)
             :
             :
             : "volatile");
    }
    if r & (1 << 0) == (1 << 0) {
        Faultmask::Inactive
    } else {
        Faultmask::Active
    }
}
