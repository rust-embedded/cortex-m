//! Base Priority Mask Register

/// Reads the CPU register
#[inline]
pub fn read() -> u8 {
    match () {
        #[cfg(target_arch = "arm")]
        () => {
            let r: u32;
            unsafe {
                asm!("mrs $0, BASEPRI" : "=r"(r) ::: "volatile");
            }
            r as u8
        }
        #[cfg(not(target_arch = "arm"))]
        () => unimplemented!(),
    }
}

/// Writes to the CPU register
#[inline]
pub unsafe fn write(_basepri: u8) {
    match () {
        #[cfg(target_arch = "arm")]
        () => asm!("msr BASEPRI, $0" :: "r"(_basepri) : "memory" : "volatile"),
        #[cfg(not(target_arch = "arm"))]
        () => unimplemented!(),
    }
}
