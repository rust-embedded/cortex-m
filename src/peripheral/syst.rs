//! SysTick: System Timer

use volatile_register::{RO, RW};

/// Registers
#[repr(C)]
pub struct Registers {
    /// Control and Status
    pub csr: RW<u32>,
    /// Reload Value
    pub rvr: RW<u32>,
    /// Current Value
    pub cvr: RW<u32>,
    /// Calibration Value
    pub calib: RO<u32>,
}
