//! CPUID

use volatile_register::RO;

/// Registers
#[repr(C)]
pub struct Registers {
    /// CPUID base
    pub base: RO<u32>,
    reserved0: [u32; 15],
    /// Processor Feature
    pub pfr: [RO<u32>; 2],
    /// Debug Feature
    pub dfr: RO<u32>,
    /// Auxiliary Feature
    pub afr: RO<u32>,
    /// Memory Model Feature
    pub mmfr: [RO<u32>; 4],
    /// Instruction Set Attribute
    pub isar: [RO<u32>; 5],
    reserved1: u32,
    /// Cache Level ID
    pub clidr: RO<u32>,
    /// Cache Type
    pub ctr: RO<u32>,
    /// Cache Size ID
    pub ccsidr: RO<u32>,
    /// Cache Size Selection
    pub csselr: RO<u32>,
}
