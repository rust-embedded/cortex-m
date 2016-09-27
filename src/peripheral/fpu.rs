//! Floating Point Unit

use volatile_register::{RO, RW};

/// Registers
#[repr(C)]
pub struct Registers {
    reserved: u32,
    /// Floating Point Context Control
    pub fpccr: RW<u32>,
    /// Floating Point Context Address
    pub fpcar: RW<u32>,
    /// Floating Point Default Status Control
    pub fpdscr: RW<u32>,
    /// Media and FP Feature
    pub mvfr: [RO<u32>; 3],
}
