//! Link register

/// Reads the CPU register
#[inline(always)]
pub fn read() -> u32 {
    let r: u32;
    unsafe {
        asm!("mov $0,R14"
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
    asm!("mov R14,$0"
         :
         : "r"(bits)
         :
         : "volatile");
}
