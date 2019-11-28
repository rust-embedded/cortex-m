//! Fault Mask Register

/// All exceptions are ...
#[allow(clippy::missing_inline_in_public_items)]
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
#[inline]
pub fn read() -> Faultmask {
    match () {
        #[cfg(cortex_m)]
        () => {
            let r = match () {
                #[cfg(feature = "inline-asm")]
                () => {
                    let r: u32;
                    unsafe { asm!("mrs $0, FAULTMASK" : "=r"(r) ::: "volatile") }
                    r
                }

                #[cfg(not(feature = "inline-asm"))]
                () => unsafe {
                    extern "C" {
                        fn __faultmask() -> u32;

                    }

                    __faultmask()
                },
            };

            if r & (1 << 0) == (1 << 0) {
                Faultmask::Inactive
            } else {
                Faultmask::Active
            }
        }

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
