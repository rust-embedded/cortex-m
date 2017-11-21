//! Debug Control Block

use volatile_register::{RW, WO};

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Debug Halting Control and Status
    pub dhcsr: RW<u32>,
    /// Debug Core Register Selector
    pub dcrsr: WO<u32>,
    /// Debug Core Register Data
    pub dcrdr: RW<u32>,
    /// Debug Exception and Monitor Control
    pub demcr: RW<u32>,
}
