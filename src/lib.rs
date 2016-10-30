//! Low level access to Cortex-M processors
//!
//! This crate provides access to:
//!
//! - Core peripherals like NVIC, SCB and SysTick.
//! - Core registers like CONTROL, MSP and PSR.
//! - Interrupt manipulation mechanisms
//! - Data structures like the vector table
//! - Miscellaneous assembly instructions like `bkpt`
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]

extern crate volatile_register;

pub mod asm;
pub mod interrupt;
pub mod peripheral;
pub mod register;

/// Stack frame
#[repr(C)]
pub struct StackFrame {
    /// (General purpose) Register 0
    pub r0: u32,
    /// (General purpose) Register 1
    pub r1: u32,
    /// (General purpose) Register 2
    pub r2: u32,
    /// (General purpose) Register 3
    pub r3: u32,
    /// (General purpose) Register 12
    pub r12: u32,
    /// Linker Register
    pub lr: u32,
    /// Program Counter
    pub pc: u32,
    /// Program Status Register
    pub xpsr: u32,
}

/// Vector Table
///
/// # References
///
/// - ARMv7-M Architecture Reference Manual (issue E.b) - Section B1.5 - ARMv7-M exception model
#[repr(C)]
pub struct VectorTable {
    /// Reset value of the Main Stack Pointer (MSP)
    pub sp_main: &'static (),
    /// Reset
    pub reset: extern "C" fn() -> !,
    /// Non Maskable Interrupt
    pub nmi: Option<Handler>,
    /// Hard Fault
    pub hard_fault: Option<Handler>,
    /// Memory Management
    pub mem_manage: Option<Handler>,
    /// Bus Fault
    pub bus_fault: Option<Handler>,
    /// Usage Fault
    pub usage_fault: Option<Handler>,
    reserved0: [u32; 4],
    /// Supervisor Call
    pub svcall: Option<Handler>,
    /// Debug Monitor
    pub debug_monitor: Option<Handler>,
    reserved1: u32,
    /// PendSV
    pub pendsv: Option<Handler>,
    /// SysTick
    pub sys_tick: Option<Handler>,
    /// Interrupts. An IMPLEMENTATION DEFINED number of them.
    pub interrupts: [Option<Handler>; 0],
}

/// Returns the vector table
pub fn vector_table() -> &'static VectorTable {
    unsafe { deref(peripheral::scb().vtor.read() as usize) }
}

/// Exception/Interrupt Handler
pub type Handler = unsafe extern "C" fn();

#[cfg(test)]
fn address<T>(r: &T) -> usize {
    r as *const T as usize
}

unsafe fn deref<T>(a: usize) -> &'static T {
    &*(a as *const T)
}

unsafe fn deref_mut<T>(a: usize) -> &'static mut T {
    &mut *(a as *mut T)
}
