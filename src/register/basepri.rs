//! Base Priority Mask Register

/// Reads the CPU register
#[inline(always)]
pub fn read() -> u8 {
    let r: u32;
    unsafe {
        asm!("mrs $0, BASEPRI"
             : "=r"(r)
             :
             :
             : "volatile");
    }
    r as u8
}

/// Writes to the CPU register
#[inline(always)]
pub unsafe fn write(basepri: u8) {
    asm!("msr BASEPRI, $0"
         :
         : "r"(basepri)
         :
         : "volatile");
}
