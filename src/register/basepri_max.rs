//! Base Priority Mask Register (conditional write)

/// Writes to BASEPRI *if*
///
/// - `basepri != 0` AND `basepri::read() == 0`, OR
/// - `basepri != 0` AND `basepri < basepri::read()`
///
/// **IMPORTANT** If you are using a Cortex-M7 device with revision r0p1 you MUST enable the
/// `cm7-r0p1` Cargo feature or this function WILL misbehave.
#[inline]
pub fn write(_basepri: u8) {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe {
            match () {
                #[cfg(not(feature = "cm7-r0p1"))]
                () => asm!("msr BASEPRI_MAX, $0" :: "r"(_basepri) : "memory" : "volatile"),
                #[cfg(feature = "cm7-r0p1")]
                () => ::interrupt::free(
                    |_| asm!("msr BASEPRI_MAX, $0" :: "r"(_basepri) : "memory" : "volatile"),
                ),
            }
        },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __basepri_max(_: u8);
            }

            __basepri_max(_basepri)
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
