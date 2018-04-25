//! Control register

/// Control register
#[derive(Clone, Copy, Debug)]
pub struct Control {
    bits: u32,
}

impl Control {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> u32 {
        self.bits
    }

    /// Thread mode privilege level
    pub fn npriv(&self) -> Npriv {
        if self.bits & (1 << 0) == (1 << 0) {
            Npriv::Unprivileged
        } else {
            Npriv::Privileged
        }
    }

    /// Currently active stack pointer
    pub fn spsel(&self) -> Spsel {
        if self.bits & (1 << 1) == (1 << 1) {
            Spsel::Psp
        } else {
            Spsel::Msp
        }
    }

    /// Whether context floating-point is currently active
    pub fn fpca(&self) -> Fpca {
        if self.bits & (1 << 2) == (1 << 2) {
            Fpca::Active
        } else {
            Fpca::NotActive
        }
    }
}

/// Thread mode privilege level
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Npriv {
    /// Privileged
    Privileged,
    /// Unprivileged
    Unprivileged,
}

impl Npriv {
    /// Is in privileged thread mode?
    pub fn is_privileged(&self) -> bool {
        *self == Npriv::Privileged
    }

    /// Is in unprivileged thread mode?
    pub fn is_unprivileged(&self) -> bool {
        *self == Npriv::Unprivileged
    }
}

/// Currently active stack pointer
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Spsel {
    /// MSP is the current stack pointer
    Msp,
    /// PSP is the current stack pointer
    Psp,
}

impl Spsel {
    /// Is MSP the current stack pointer?
    pub fn is_msp(&self) -> bool {
        *self == Spsel::Msp
    }

    /// Is PSP the current stack pointer?
    pub fn is_psp(&self) -> bool {
        *self == Spsel::Psp
    }
}

/// Whether context floating-point is currently active
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Fpca {
    /// Floating-point context active.
    Active,
    /// No floating-point context active
    NotActive,
}

impl Fpca {
    /// Is a floating-point context active?
    pub fn is_active(&self) -> bool {
        *self == Fpca::Active
    }

    /// Is a floating-point context not active?
    pub fn is_not_active(&self) -> bool {
        *self == Fpca::NotActive
    }
}

/// Reads the CPU register
#[inline]
pub fn read() -> Control {
    match () {
        #[cfg(cortex_m)]
        () => {
            let r = match () {
                #[cfg(feature = "inline-asm")]
                () => {
                    let r: u32;
                    unsafe { asm!("mrs $0, CONTROL" : "=r"(r) ::: "volatile") }
                    r
                }

                #[cfg(not(feature = "inline-asm"))]
                () => unsafe {
                    extern "C" {
                        fn __control() -> u32;
                    }

                    __control()
                },
            };

            Control { bits: r }
        }

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
