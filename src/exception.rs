//! Exceptions

/// Enumeration of all the exception types
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Exception {
    /// Non-maskable interrupt
    NMI,
    /// Other type of faults and unhandled faults
    HardFault,
    /// Memory protection related fault
    MemManage,
    /// Pre-fetch or memory access fault
    BusFault,
    /// Fault due to undefined instruction or illegal state
    UsageFault,
    /// Supervisor call
    SVCall,
    /// Pendable request for system-level service
    PendSV,
    /// System timer exception
    SysTick,
    /// An interrupt
    Interrupt(u8),
    // Unreachable variant
    #[doc(hidden)] Reserved,
}

impl Exception {
    /// Returns the type of the exception that's currently active
    ///
    /// Returns `None` if no exception is currently active
    pub fn active() -> Option<Exception> {
        // NOTE(safe) atomic read with no side effects
        let icsr = unsafe { (*::peripheral::SCB::ptr()).icsr.read() };

        Some(match icsr as u8 {
            0 => return None,
            2 => Exception::NMI,
            3 => Exception::HardFault,
            4 => Exception::MemManage,
            5 => Exception::BusFault,
            6 => Exception::UsageFault,
            11 => Exception::SVCall,
            14 => Exception::PendSV,
            15 => Exception::SysTick,
            n if n >= 16 => Exception::Interrupt(n - 16),
            _ => Exception::Reserved,
        })
    }
}

/// Registers stacked (pushed into the stack) during an exception
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ExceptionFrame {
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
