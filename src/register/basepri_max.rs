//! Base Priority Mask Register (conditional write)

/// Writes to BASEPRI *if*
///
/// - `basepri != 0` AND `basepri::read() == 0`, OR
/// - `basepri != 0` AND `basepri < basepri::read()`
///
/// **IMPORTANT** If you are using a Cortex-M7 device with revision r0p1 you MUST enable the
/// `cm7-r0p1` Cargo feature or this function WILL misbehave.
#[cfg_attr(not(target_arch = "arm"), allow(unused_variables))]
#[inline]
pub fn write(basepri: u8) {
    match () {
        #[cfg(target_arch = "arm")]
        () => unsafe {
            match () {
                #[cfg(not(feature = "cm7-r0p1"))]
                () => asm!("msr BASEPRI_MAX, $0" :: "r"(basepri) : "memory" : "volatile"),
                #[cfg(feature = "cm7-r0p1")]
                () => asm!("cpsid i
                            msr BASEPRI_MAX, $0
                            cpsie i" :: "r"(basepri) : "memory" : "volatile"),
            }
        },
        #[cfg(not(target_arch = "arm"))]
        () => unimplemented!(),
    }
}
