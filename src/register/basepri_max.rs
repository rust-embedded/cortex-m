//! Base Priority Mask Register (conditional write)

/// Writes to BASEPRI *if*
///
/// - `basepri != 0` AND `basepri::read() == 0`, OR
/// - `basepri != 0` AND `basepri < basepri::read()`
#[inline(always)]
pub fn write(basepri: u8) {
    unsafe {
        asm!("msr BASEPRI_MAX, $0"
             :
             : "r"(basepri as u32)
             :
             : "volatile");
    }
}
