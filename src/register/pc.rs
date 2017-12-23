//! Program counter

/// Reads the CPU register
#[inline]
pub fn read() -> u32 {
    match () {
        #[cfg(target_arch = "arm")]
        () => {
            let r;
            unsafe { asm!("mov $0,R15" : "=r"(r) ::: "volatile") }
            r
        }
        #[cfg(not(target_arch = "arm"))]
        () => unimplemented!(),
    }
}

/// Writes `bits` to the CPU register
#[cfg_attr(not(target_arch = "arm"), allow(unused_variables))]
#[inline]
pub unsafe fn write(bits: u32) {
    match () {
        #[cfg(target_arch = "arm")]
        () => asm!("mov R15,$0" :: "r"(bits) :: "volatile"),
        #[cfg(not(target_arch = "arm"))]
        () => unimplemented!(),
    }
}
