//! Base Priority Mask Register

/// Reads the CPU register
#[inline]
pub fn read() -> u8 {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => {
            let r: u32;
            unsafe {
                asm!("mrs $0, BASEPRI" : "=r"(r) ::: "volatile");
            }
            r as u8
        }

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __basepri_r() -> u8;
            }

            __basepri_r()
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Writes to the CPU register
///
/// **IMPORTANT** If you are using a Cortex-M7 device with revision r0p1 you MUST enable the
/// `cm7-r0p1` Cargo feature or this function WILL misbehave.
#[inline]
pub unsafe fn write(_basepri: u8) {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => match () {
            #[cfg(not(feature = "cm7-r0p1"))]
            () => asm!("msr BASEPRI, $0" :: "r"(_basepri) : "memory" : "volatile"),
            #[cfg(feature = "cm7-r0p1")]
            () => ::interrupt::free(
                |_| asm!("msr BASEPRI, $0" :: "r"(_basepri) : "memory" : "volatile"),
            ),
        },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => match () {
            #[cfg(not(feature = "cm7-r0p1"))]
            () => {
                extern "C" {
                    fn __basepri_w(_: u8);
                }

                __basepri_w(_basepri);
            }
            #[cfg(feature = "cm7-r0p1")]
            () => {
                extern "C" {
                    fn __basepri_w_cm7_r0p1(_: u8);
                }

                __basepri_w_cm7_r0p1(_basepri);
            }
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
