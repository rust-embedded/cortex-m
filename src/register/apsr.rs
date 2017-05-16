//! Application Program Status Register

/// Application Program Status Register
#[derive(Clone, Copy, Debug)]
pub struct Apsr {
    bits: u32,
}

impl Apsr {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> u32 {
        self.bits
    }

    /// DSP overflow and saturation flag
    pub fn q(&self) -> bool {
        self.bits & (1 << 27) == (1 << 27)
    }

    /// Overflow flag
    pub fn v(&self) -> bool {
        self.bits & (1 << 28) == (1 << 28)
    }

    /// Carry or borrow flag
    pub fn c(&self) -> bool {
        self.bits & (1 << 29) == (1 << 29)
    }

    /// Zero flag
    pub fn z(&self) -> bool {
        self.bits & (1 << 30) == (1 << 30)
    }

    /// Negative flag
    pub fn n(&self) -> bool {
        self.bits & (1 << 31) == (1 << 31)
    }
}

/// Reads the CPU register
#[inline(always)]
pub fn read() -> Apsr {
    let r: u32;
    unsafe {
        asm!("mrs $0, APSR"
             : "=r"(r)
             :
             :
             : "volatile");
    }
    Apsr { bits: r }
}
