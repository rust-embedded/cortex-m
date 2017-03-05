//! Priority mask register

/// All exceptions with configurable priority are ...
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Primask {
    /// Active
    Active,
    /// Inactive
    Inactive,
}

impl Primask {
    /// All exceptions with configurable priority are active
    pub fn is_active(&self) -> bool {
        *self == Primask::Active
    }

    /// All exceptions with configurable priority are inactive
    pub fn is_inactive(&self) -> bool {
        *self == Primask::Inactive
    }
}

/// Reads the CPU register
#[inline(always)]
pub fn read() -> Primask {
    let r: u32;
    unsafe {
        asm!("mrs $0, PRIMASK"
             : "=r"(r)
             :
             :
             : "volatile");
    }
    if r & (1 << 0) == (1 << 0) {
        Primask::Inactive
    } else {
        Primask::Active
    }
}
