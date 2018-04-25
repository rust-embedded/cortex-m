//! Program counter

/// Reads the CPU register
#[inline]
pub fn read() -> u32 {
    match () {
        #[cfg(cortex_m)]
        () => {
            let r;
            unsafe { asm!("mov $0,R15" : "=r"(r) ::: "volatile") }
            r
        }

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Writes `bits` to the CPU register
#[inline]
pub unsafe fn write(_bits: u32) {
    match () {
        #[cfg(cortex_m)]
        () => asm!("mov R15,$0" :: "r"(_bits) :: "volatile"),

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
