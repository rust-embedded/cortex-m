//! Data Watchpoint and Trace unit

#[cfg(not(armv6m))]
use volatile_register::WO;
use volatile_register::{RO, RW};

use crate::peripheral::DWT;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Control
    pub ctrl: RW<u32>,
    /// Cycle Count
    #[cfg(not(armv6m))]
    pub cyccnt: RW<u32>,
    /// CPI Count
    #[cfg(not(armv6m))]
    pub cpicnt: RW<u32>,
    /// Exception Overhead Count
    #[cfg(not(armv6m))]
    pub exccnt: RW<u32>,
    /// Sleep Count
    #[cfg(not(armv6m))]
    pub sleepcnt: RW<u32>,
    /// LSU Count
    #[cfg(not(armv6m))]
    pub lsucnt: RW<u32>,
    /// Folded-instruction Count
    #[cfg(not(armv6m))]
    pub foldcnt: RW<u32>,
    /// Cortex-M0(+) does not have  these parts
    #[cfg(armv6m)]
    reserved: [u32; 6],
    /// Program Counter Sample
    pub pcsr: RO<u32>,
    /// Comparators
    #[cfg(armv6m)]
    pub c: [Comparator; 2],
    #[cfg(not(armv6m))]
    /// Comparators
    pub c: [Comparator; 16],
    #[cfg(not(armv6m))]
    reserved: [u32; 932],
    /// Lock Access
    #[cfg(not(armv6m))]
    pub lar: WO<u32>,
    /// Lock Status
    #[cfg(not(armv6m))]
    pub lsr: RO<u32>,
}

/// Comparator
#[repr(C)]
pub struct Comparator {
    /// Comparator
    pub comp: RW<u32>,
    /// Comparator Mask
    pub mask: RW<u32>,
    /// Comparator Function
    pub function: RW<u32>,
    reserved: u32,
}

impl DWT {
    /// Enables the cycle counter
    #[cfg(not(armv6m))]
    pub fn enable_cycle_counter(&mut self) {
        unsafe { self.ctrl.modify(|r| r | 1) }
    }

    /// Returns the current clock cycle count
    #[cfg(not(armv6m))]
    pub fn get_cycle_count() -> u32 {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::ptr()).cyccnt.read() }
    }
}
