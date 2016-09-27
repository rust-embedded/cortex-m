//! Flash Patch and Breakpoint unit

use volatile_register::{RO, RW, WO};

/// Registers
#[repr(C)]
pub struct Registers {
    /// Control
    pub ctrl: RW<u32>,
    /// Remap
    pub remap: RW<u32>,
    /// Comparator
    pub comp: [RW<u32>; 127],
    reserved: [u32; 875],
    /// Lock Access
    pub lar: WO<u32>,
    /// Lock Status
    pub lsr: RO<u32>,
}
