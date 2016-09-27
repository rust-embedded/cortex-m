//! Nested Vector Interrupt Controller

use volatile_register::{RO, RW};

/// Registers
#[repr(C)]
pub struct Registers {
    /// Interrupt Set-Enable
    pub iser: [RW<u32>; 16],
    reserved0: [u32; 16],
    /// Interrupt Clear-Enable
    pub icer: [RW<u32>; 16],
    reserved1: [u32; 16],
    /// Interrupt Set-Pending
    pub ispr: [RW<u32>; 16],
    reserved2: [u32; 16],
    /// Interrupt Clear-Pending
    pub icpr: [RW<u32>; 16],
    reserved3: [u32; 16],
    /// Interrupt Active Bit
    pub iabr: [RO<u32>; 16],
    reserved4: [u32; 48],
    /// Interrupt Priority
    pub ipr: [RW<u32>; 124],
}
