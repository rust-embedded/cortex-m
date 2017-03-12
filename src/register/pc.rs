//! Program counter

/// Reads the CPU register
#[inline(always)]
pub fn read() -> u32 {
    let r;
    unsafe {
        asm!("mov $0,R15"
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
    asm!("mov R15,$0"
         :
         : "r"(bits)
         :
         : "volatile");
}
