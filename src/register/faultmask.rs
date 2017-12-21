//! Fault Mask Register

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

    #[cfg(target_arch = "arm")]
    unsafe {
        asm!("mrs $0, FAULTMASK"
             : "=r"(r)
             :
             :
             : "volatile");
    }

    #[cfg(not(target_arch = "arm"))]
    { r = 0; }

    if r & (1 << 0) == (1 << 0) {
        Faultmask::Inactive
    } else {
        Faultmask::Active
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_should_compile() {
        // Make sure that ARM-specific inline assembly is only included on ARM.
        super::read();
    }
}
