//! Debug Control Block

use volatile_register::{RW, WO};

use peripheral::DCB;

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

impl DCB {
    /// Enables TRACE. This is for example required by the 
    /// `peripheral::DWT` cycle counter to work properly.
    /// As by STM documentation, this flag is not reset on
    /// soft-reset, only on power reset.
    pub fn enable_trace(&mut self) {
        unsafe { self.demcr.modify(|w| w | 0x01000000); }
    }
    
    /// Disables TRACE. See `DCB::enable_trace()` for more details
    pub fn disable_trace(&mut self) {
        unsafe { self.demcr.modify(|w| w & !0x01000000); }
    }
}
