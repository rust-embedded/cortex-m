//! Memory Protection Unit

use volatile_register::{RO, RW};

/// Registers
#[repr(C)]
pub struct Registers {
    /// Type
    pub _type: RO<u32>,
    /// Control
    pub ctrl: RW<u32>,
    /// Region Number
    pub rnr: RW<u32>,
    /// Region Base Address
    pub rbar: RW<u32>,
    /// Region Attribute and Size
    pub rasr: RW<u32>,
    /// Alias 1 of RBAR
    pub rbar_a1: RW<u32>,
    /// Alias 1 of RSAR
    pub rsar_a1: RW<u32>,
    /// Alias 2 of RBAR
    pub rbar_a2: RW<u32>,
    /// Alias 2 of RSAR
    pub rsar_a2: RW<u32>,
    /// Alias 3 of RBAR
    pub rbar_a3: RW<u32>,
    /// Alias 3 of RSAR
    pub rsar_a3: RW<u32>,
}
