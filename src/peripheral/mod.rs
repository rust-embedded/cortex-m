//! Core peripherals
//!
//! # Notes
//!
//! - Although the `*_mut()` functions always return a valid/live reference, the API doesn't prevent
//!   the user from creating multiple mutable aliases. It's up to the user to ensure that no
//!   unsynchonized concurrent access is performed through these references.
//!
//! # Caveats
//!
//! - The API doesn't check if the value passed to `write` is valid (e.g. reserved bits are not
//!   modified) or not. It's up to the user to verify that.
//!
//! # References
//!
//! - ARMv7-M Architecture Reference Manual (Issue E.b) - Chapter B3

pub mod cpuid;
pub mod dcb;
pub mod dwt;
pub mod fpb;
pub mod fpu;
pub mod itm;
pub mod mpu;
pub mod nvic;
pub mod scb;
pub mod syst;
pub mod tpiu;

mod test;

const CPUID: usize = 0xE000_ED00;
const DCB: usize = 0xE000_EDF0;
const DWT: usize = 0xE000_1000;
const FPB: usize = 0xE000_2000;
const FPU: usize = 0xE000_EF30;
const ITM: usize = 0xE000_0000;
const MPU: usize = 0xE000_ED90;
const NVIC: usize = 0xE000_E100;
const SCB: usize = 0xE000_ED04;
const SYST: usize = 0xE000_E010;
const TPIU: usize = 0xE004_0000;

// TODO stand-alone registers: ICTR, ACTLR and STIR

/// `&cpuid::Registers`
pub fn cpuid() -> &'static cpuid::Registers {
    unsafe { ::deref(CPUID) }
}

/// `&dcb::Registers`
pub fn dcb() -> &'static dcb::Registers {
    unsafe { ::deref(DCB) }
}

/// `&mut dcb::Registers`
pub unsafe fn dcb_mut() -> &'static mut dcb::Registers {
    ::deref_mut(DCB)
}

/// `&dwt::Registers`
pub fn dwt() -> &'static dwt::Registers {
    unsafe { ::deref(DWT) }
}

/// `&mut dwt::Registers`
pub unsafe fn dwt_mut() -> &'static mut dwt::Registers {
    ::deref_mut(DWT)
}

/// `&fpb::Registers`
pub fn fpb() -> &'static fpb::Registers {
    unsafe { ::deref(FPB) }
}

/// `&mut fpb::Registers`
pub unsafe fn fpb_mut() -> &'static mut fpb::Registers {
    ::deref_mut(FPB)
}

/// `&fpu::Registers`
pub fn fpu() -> &'static fpu::Registers {
    unsafe { ::deref(FPU) }
}

/// `&mut fpu::Registers`
pub unsafe fn fpu_mut() -> &'static mut fpu::Registers {
    ::deref_mut(FPU)
}

/// `&itm::Registers`
pub fn itm() -> &'static itm::Registers {
    unsafe { ::deref(ITM) }
}

/// `&mut itm::Registers`
pub unsafe fn itm_mut() -> &'static mut itm::Registers {
    ::deref_mut(ITM)
}

/// `&mpu::Registers`
pub fn mpu() -> &'static mpu::Registers {
    unsafe { ::deref(MPU) }
}

/// `&mut mpu::Registers`
pub unsafe fn mpu_mut() -> &'static mut mpu::Registers {
    ::deref_mut(MPU)
}

/// `&nvic::Registers`
pub fn nvic() -> &'static nvic::Registers {
    unsafe { ::deref(NVIC) }
}

/// `&mut nvic::Registers`
pub unsafe fn nvic_mut() -> &'static mut nvic::Registers {
    ::deref_mut(NVIC)
}

/// `&scb::Registers`
pub fn scb() -> &'static scb::Registers {
    unsafe { ::deref(SCB) }
}

/// `&mut scb::Registers`
pub unsafe fn scb_mut() -> &'static mut scb::Registers {
    ::deref_mut(SCB)
}

/// `&syst::Registers`
pub fn syst() -> &'static syst::Registers {
    unsafe { ::deref(SYST) }
}

/// `&mut syst::Registers`
pub unsafe fn syst_mut() -> &'static mut syst::Registers {
    ::deref_mut(SYST)
}

/// `&tpiu::Registers`
pub fn tpiu() -> &'static tpiu::Registers {
    unsafe { ::deref(TPIU) }
}

/// `&mut tpiu::Registers`
pub unsafe fn tpiu_mut() -> &'static mut tpiu::Registers {
    ::deref_mut(TPIU)
}
