//! Exceptions

#![allow(non_camel_case_types)]

/// Enumeration of all exceptions
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Exception {
    /// Non-maskable interrupt
    Nmi,
    /// All class of fault.
    HardFault,
    /// Memory management.
    MenManage,
    /// Pre-fetch fault, memory access fault.
    BusFault,
    /// Undefined instruction or illegal state.
    UsageFault,
    /// System service call via SWI instruction
    Svcall,
    /// Pendable request for system service
    Pendsv,
    /// System tick timer
    SysTick,
    /// An interrupt
    Interrupt(u8),
    // Unreachable variant
    #[doc(hidden)]
    Reserved,
}

impl Exception {
    /// Returns the kind of exception that's currently being serviced
    pub fn active() -> Option<Exception> {
        // NOTE(safe) atomic read
        let icsr = unsafe { (*::peripheral::SCB.get()).icsr.read() };
        if icsr == 0 {
            return None;
        }

        Some(match icsr as u8 {
            2 => Exception::Nmi,
            3 => Exception::HardFault,
            4 => Exception::MenManage,
            5 => Exception::BusFault,
            6 => Exception::UsageFault,
            11 => Exception::Svcall,
            14 => Exception::Pendsv,
            15 => Exception::SysTick,
            n if n >= 16 => Exception::Interrupt(n - 16),
            _ => Exception::Reserved,
        })
    }
}

/// Registers stacked during an exception
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct StackedRegisters {
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
