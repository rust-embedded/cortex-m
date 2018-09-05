//! Debug Control Block

use volatile_register::{RW, WO};

use peripheral::DCB;

const BIT_TRACENA: u32 = 0x01 << 24;

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
        // set bit 24 / TRACENA
        unsafe { self.demcr.modify(|w| w | BIT_TRACENA); }
    }
    
    /// Disables TRACE. See `DCB::enable_trace()` for more details
    pub fn disable_trace(&mut self) {
        // unset bit 24 / TRACENA
        unsafe { self.demcr.modify(|w| w & !BIT_TRACENA); }
    }
}
