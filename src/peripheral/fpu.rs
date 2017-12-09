//! Floating Point Unit

#[cfg(any(has_fpu, test))]
use volatile_register::{RO, RW};

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    reserved: u32,
    /// Floating Point Context Control
    #[cfg(any(has_fpu, test))]
    pub fpccr: RW<u32>,
    /// Floating Point Context Address
    #[cfg(any(has_fpu, test))]
    pub fpcar: RW<u32>,
    /// Floating Point Default Status Control
    #[cfg(any(has_fpu, test))]
    pub fpdscr: RW<u32>,
    /// Media and FP Feature
    #[cfg(any(has_fpu, test))]
    pub mvfr: [RO<u32>; 3],
}
