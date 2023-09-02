//! Debug Control Block

use volatile_register::{RW, WO};

use crate::peripheral::DCB;
use core::ptr;

const DCB_DEMCR_TRCENA: u32 = 1 << 24;
const DCB_DEMCR_MON_EN: u32 = 1 << 16;

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
    /// Global enable for all [`DWT`](crate::peripheral::DWT) and
    /// [`ITM`](crate::peripheral::ITM) features.
    ///
    /// Note: vendor-specific registers may have to be set to completely
    /// enable tracing. For example, on the STM32F401RE, `TRACE_MODE`
    /// and `TRACE_IOEN` must be configured in `DBGMCU_CR` register.
    #[inline]
    pub fn enable_trace(&mut self) {
        // set bit 24 / TRCENA
        unsafe {
            self.demcr.modify(|w| w | DCB_DEMCR_TRCENA);
        }
    }

    /// Disables TRACE. See `DCB::enable_trace()` for more details
    #[inline]
    pub fn disable_trace(&mut self) {
        // unset bit 24 / TRCENA
        unsafe {
            self.demcr.modify(|w| w & !DCB_DEMCR_TRCENA);
        }
    }

    /// Enables the [`DebugMonitor`](crate::peripheral::scb::Exception::DebugMonitor) exception
    #[inline]
    pub fn enable_debug_monitor(&mut self) {
        unsafe {
            self.demcr.modify(|w| w | DCB_DEMCR_MON_EN);
        }
    }

    /// Disables the [`DebugMonitor`](crate::peripheral::scb::Exception::DebugMonitor) exception
    #[inline]
    pub fn disable_debug_monitor(&mut self) {
        unsafe {
            self.demcr.modify(|w| w & !DCB_DEMCR_MON_EN);
        }
    }

    /// Is there a debugger attached? (see note)
    ///
    /// Note: This function is [reported not to
    /// work](http://web.archive.org/web/20180821191012/https://community.nxp.com/thread/424925#comment-782843)
    /// on Cortex-M0 devices. Per the ARM v6-M Architecture Reference Manual, "Access to the DHCSR
    /// from software running on the processor is IMPLEMENTATION DEFINED". Indeed, from the
    /// [Cortex-M0+ r0p1 Technical Reference Manual](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0484c/BABJHEIG.html), "Note Software cannot access the debug registers."
    #[inline]
    pub fn is_debugger_attached() -> bool {
        unsafe {
            // do an 8-bit read of the 32-bit DHCSR register, and get the LSB
            let value = ptr::read_volatile(Self::PTR as *const u8);
            value & 0x1 == 1
        }
    }
}
