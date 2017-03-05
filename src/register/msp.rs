//! Main Stack Pointer

/// Reads the CPU register
#[inline(always)]
pub fn read() -> u32 {
    let r;
    unsafe {
        asm!("mrs $0,MSP"
             : "=r"(r)
             :
             :
             : "volatile");
    }
    r
}

/// Writes `bits` to the CPU register
#[inline(always)]
pub unsafe fn write(bits: u32) {
    asm!("msr MSP,$0"
         :
         : "r"(bits)
         :
         : "volatile");
}
