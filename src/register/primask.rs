//! Priority mask register

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
    pub fn is_active(self) -> bool {
        self == Primask::Active
    }

    /// All exceptions with configurable priority are inactive
    pub fn is_inactive(self) -> bool {
        self == Primask::Inactive
    }
}

/// Reads the CPU register
#[inline]
pub fn read() -> Primask {
    match () {
        #[cfg(cortex_m)]
        () => {
            let r = match () {
                #[cfg(feature = "inline-asm")]
                () => {
                    let r: u32;
                    unsafe { asm!("mrs $0, PRIMASK" : "=r"(r) ::: "volatile") }
                    r
                }

                #[cfg(not(feature = "inline-asm"))]
                () => {
                    extern "C" {
                        fn __primask() -> u32;
                    }

                    unsafe { __primask() }
                }
            };

            if r & (1 << 0) == (1 << 0) {
                Primask::Inactive
            } else {
                Primask::Active
            }
        }

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
