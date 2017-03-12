//! Process Stack Pointer

/// Reads the CPU register
#[inline(always)]
pub fn read() -> u32 {
    let r;
    unsafe {
        asm!("mrs $0,PSP"
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
    asm!("msr PSP,$0"
         :
         : "r"(bits)
         :
         : "volatile");
}
