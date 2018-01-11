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
///
/// **IMPORTANT** If you are using a Cortex-M7 device with revision r0p1 you MUST enable the
/// `cm7-r0p1` Cargo feature or this function WILL misbehave.
#[cfg_attr(not(target_arch = "arm"), allow(unused_variables))]
#[inline]
pub unsafe fn write(basepri: u8) {
    match () {
        #[cfg(target_arch = "arm")]
        () => match () {
            #[cfg(not(feature = "cm7-r0p1"))]
            () => asm!("msr BASEPRI, $0" :: "r"(basepri) : "memory" : "volatile"),
            #[cfg(feature = "cm7-r0p1")]
            () => asm!("cpsid i
                        msr BASEPRI, $0
                        cpsie i" :: "r"(basepri) : "memory" : "volatile"),
        },
        #[cfg(not(target_arch = "arm"))]
        () => unimplemented!(),
    }
}
