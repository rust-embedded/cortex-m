//! Trace Port Interface Unit

use volatile_register::{RO, RW, WO};

/// Registers
#[repr(C)]
pub struct Registers {
    /// Supported Parallel Port Sizes
    pub sspsr: RO<u32>,
    /// Current Parallel Port Size
    pub cspsr: RW<u32>,
    reserved0: [u32; 2],
    /// Asynchronous Clock Prescaler
    pub acpr: RW<u32>,
    reserved1: [u32; 55],
    /// Selected Pin Control
    pub sppr: RW<u32>,
    reserved2: [u32; 943],
    /// Lock Access
    pub lar: WO<u32>,
    /// Lock Status
    pub lsr: RO<u32>,
    reserved3: [u32; 4],
    /// TPIU Type
    pub _type: RO<u32>,
}
