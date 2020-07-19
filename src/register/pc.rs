//! Program counter

/// Reads the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[inline]
pub fn read() -> u32 {
    match () {
        #[cfg(cortex_m)]
        () => {
            let r;
            unsafe { llvm_asm!("mov $0,R15" : "=r"(r) ::: "volatile") }
            r
        }

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Writes `bits` to the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[inline]
pub unsafe fn write(_bits: u32) {
    match () {
        #[cfg(cortex_m)]
        () => llvm_asm!("mov R15,$0" :: "r"(_bits) :: "volatile"),

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
