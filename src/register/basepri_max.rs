//! Base Priority Mask Register (conditional write)

/// Writes to BASEPRI *if*
///
/// - `basepri != 0` AND `basepri::read() == 0`, OR
/// - `basepri != 0` AND `basepri < basepri::read()`
#[inline]
pub fn write(_basepri: u8) {
    match () {
        #[cfg(target_arch = "arm")]
        () => unsafe {
            asm!("msr BASEPRI_MAX, $0" :: "r"(_basepri) : "memory" : "volatile");
        },
        #[cfg(not(target_arch = "arm"))]
        () => unimplemented!(),
    }
}
