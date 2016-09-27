//! System Control Block

use volatile_register::RW;

/// Registers
#[repr(C)]
pub struct Registers {
    /// Interrupt Control and State
    pub icsr: RW<u32>,
    /// Vector Table Offset
    pub vtor: RW<u32>,
    /// Application Interrupt and Reset Control
    pub aircr: RW<u32>,
    /// System Control
    pub scr: RW<u32>,
    /// Configuration and Control
    pub ccr: RW<u32>,
    /// System Handler Priority
    pub shpr: [RW<u8>; 12],
    /// System Handler Control and State
    pub shpcrs: RW<u32>,
    /// Configurable Fault Status
    pub cfsr: RW<u32>,
    /// HardFault Status
    pub hfsr: RW<u32>,
    /// Debug Fault Status
    pub dfsr: RW<u32>,
    /// MemManage Fault Address
    pub mmar: RW<u32>,
    /// BusFault Address
    pub bfar: RW<u32>,
    /// Auxiliary Fault Status
    pub afsr: RW<u32>,
    reserved: [u32; 18],
    /// Coprocessor Access Control
    pub cpacr: RW<u32>,
}
