//! Floating-point Status Control Register

/// Floating-point Status Control Register
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Clone, Copy, Debug)]
pub struct Fpscr {
    bits: u32,
}

/// Rounding mode
#[derive(Clone, Copy, Debug)]
pub enum RMode {
    Nearest,
    PlusInfinity,
    MinusInfinity,
    Zero,
}

impl Fpscr {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(self) -> u32 {
        self.bits
    }

    /// Read the Negative condition code flag
    #[inline]
    pub fn n(self) -> bool {
        self.bits & (1 << 31) != 0
    }

    /// Read the Zero condition code flag
    #[inline]
    pub fn z(self) -> bool {
        self.bits & (1 << 30) != 0
    }

    /// Read the Carry condition code flag
    #[inline]
    pub fn c(self) -> bool {
        self.bits & (1 << 29) != 0
    }

    /// Read the Overflow condition code flag
    #[inline]
    pub fn v(self) -> bool {
        self.bits & (1 << 28) != 0
    }

    /// Read the Alternative Half Precision bit
    #[inline]
    pub fn ahp(self) -> bool {
        if self.bits & (1 << 26) != 0
    }

    /// Read the Default NaN mode bit
    #[inline]
    pub fn dn(self) -> bool {
        if self.bits & (1 << 25) != 0
    }

    /// Read the Flush to Zero mode bit
    #[inline]
    pub fn fz(self) -> bool {
        if self.bits & (1 << 24) != 0
    }

    /// Read the Rounding Mode control field
    #[inline]
    pub fn rmode(self) -> RMode {
        match self.bits & (3 << 22) {
            0 << 22 => RMode::Nearest,
            1 << 22 => RMode::PlusInfinity,
            2 << 22 => RMode::MinusInfinity,
            3 << 22 => RMode::Zero,
        }
    }

    /// Read the Input Denormal cumulative exception bit
    #[inline]
    pub fn idc(self) -> bool {
        if self.bits & (1 << 7) != 0
    }

    /// Read the Inexact cumulative exception bit
    #[inline]
    pub fn ixc(self) -> bool {
        if self.bits & (1 << 4) != 0
    }

    /// Read the Underflow cumulative exception bit
    #[inline]
    pub fn ufc(self) -> bool {
        if self.bits & (1 << 3) != 0
    }

    /// Read the Overflow cumulative exception bit
    #[inline]
    pub fn ofc(self) -> bool {
        if self.bits & (1 << 2) != 0
    }

    /// Read the Division by Zero cumulative exception bit
    #[inline]
    pub fn dzc(self) -> bool {
        if self.bits & (1 << 1) != 0
    }

    /// Read the Invalid Operation cumulative exception bit
    #[inline]
    pub fn ioc(self) -> bool {
        if self.bits & (1 << 0) != 0
    }
}

/// Read the FPSCR register
pub fn read() -> Fpscr {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => {
            let r: u32;
            unsafe {
                llvm_asm!("vmrs $0, fpscr" : "=r"(r) ::: "volatile");
            }
            Fpscr { bits: r }
        }

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __get_FPSCR() -> u32;
            }

            Fpscr { bits: __get_FPSCR() }
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Set the value of the FPSCR register
pub unsafe fn write(value: u32) {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => {
            unsafe {
                llvm_asm!("vmsr fpscr, $0" :: "r"(value) :: "volatile");
            }
        }

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __set_FPSCR(value: u32);
            }

            __set_FPSCR(value);
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
