//! Base Priority Mask Register

/// Reads the CPU register
#[inline(always)]
pub fn read() -> u8 {
    let r: u32;

    #[cfg(target_arch = "arm")]
    unsafe {
        asm!("mrs $0, BASEPRI"
             : "=r"(r)
             :
             :
             : "volatile");
    }

    #[cfg(not(target_arch = "arm"))]
    { r = 0; }

    r as u8
}

/// Writes to the CPU register
#[inline(always)]
pub unsafe fn write(_basepri: u8) {
    #[cfg(target_arch = "arm")]
    asm!("msr BASEPRI, $0"
         :
         : "r"(_basepri)
         : "memory"
         : "volatile");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_should_compile() {
        // Make sure that ARM-specific inline assembly is only included on ARM.
        super::read();
        unsafe { super::write(5); }
    }
}
