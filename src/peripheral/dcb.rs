//! Debug Control Block

use volatile_register::{RW, WO};

use peripheral::DCB;

const DCB_DEMCR_TRCENA: u32 = 1 << 24;

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
        // set bit 24 / TRCENA
        unsafe { self.demcr.modify(|w| w | DCB_DEMCR_TRCENA); }
    }

    /// Disables TRACE. See `DCB::enable_trace()` for more details
    pub fn disable_trace(&mut self) {
        // unset bit 24 / TRCENA
        unsafe { self.demcr.modify(|w| w & !DCB_DEMCR_TRCENA); }

    /// Is there a debugger attached?
    pub fn is_debugger_attached(&self) -> bool {
        self.dhcsr.read() & 0x1 == 1
    }
}
