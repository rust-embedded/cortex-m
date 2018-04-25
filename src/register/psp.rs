//! Process Stack Pointer

/// Reads the CPU register
#[inline]
pub fn read() -> u32 {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => {
            let r;
            unsafe { asm!("mrs $0,PSP" : "=r"(r) ::: "volatile") }
            r
        }

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __psp_r() -> u32;
            }

            __psp_r()
        }

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Writes `bits` to the CPU register
#[inline]
pub unsafe fn write(_bits: u32) {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => asm!("msr PSP,$0" :: "r"(_bits) :: "volatile"),

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => {
            extern "C" {
                fn __psp_w(_: u32);
            }

            __psp_w(_bits);
        }

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
