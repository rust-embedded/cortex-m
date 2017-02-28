//! Exceptions

use {Handler, Reserved, StackFrame};

/// Kind of exception
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Exception {
    /// i.e. currently not servicing an exception
    ThreadMode,
    /// Non-maskable interrupt.
    Nmi,
    /// All class of fault.
    HardFault,
    /// Memory management.
    MemoryManagementFault,
    /// Pre-fetch fault, memory access fault.
    BusFault,
    /// Undefined instruction or illegal state.
    UsageFault,
    /// System service call via SWI instruction
    SVCall,
    /// Pendable request for system service
    PendSV,
    /// System tick timer
    Systick,
    /// An interrupt
    Interrupt(u8),
    // Unreachable variant
    #[doc(hidden)]
    Reserved,
}

impl Exception {
    /// Returns the kind of exception that's currently being serviced
    pub fn current() -> Exception {
        match ::peripheral::scb().icsr.read() as u8 {
            0 => Exception::ThreadMode,
            2 => Exception::Nmi,
            3 => Exception::HardFault,
            4 => Exception::MemoryManagementFault,
            5 => Exception::BusFault,
            6 => Exception::UsageFault,
            11 => Exception::SVCall,
            14 => Exception::PendSV,
            15 => Exception::Systick,
            n if n >= 16 => Exception::Interrupt(n - 16),
            _ => Exception::Reserved,
        }
    }
}

/// Exception handlers
#[repr(C)]
pub struct Handlers {
    /// Non-maskable interrupt
    pub nmi: Handler,
    /// All class of fault
    pub hard_fault: Handler,
    /// Memory management
    pub mem_manage: Handler,
    /// Pre-fetch fault, memory access fault
    pub bus_fault: Handler,
    /// Undefined instruction or illegal state
    pub usage_fault: Handler,
    /// Reserved spots in the vector table
    pub _reserved0: [Reserved; 4],
    /// System service call via SWI instruction
    pub svcall: Handler,
    /// Reserved spots in the vector table
    pub _reserved1: [Reserved; 2],
    /// Pendable request for system service
    pub pendsv: Handler,
    /// System tick timer
    pub sys_tick: Handler,
}

/// Default exception handlers
pub const DEFAULT_HANDLERS: Handlers = Handlers {
    _reserved0: [Reserved::Vector; 4],
    _reserved1: [Reserved::Vector; 2],
    bus_fault: default_handler,
    hard_fault: default_handler,
    mem_manage: default_handler,
    nmi: default_handler,
    pendsv: default_handler,
    svcall: default_handler,
    sys_tick: default_handler,
    usage_fault: default_handler,
};

/// The default exception handler
///
/// This handler triggers a breakpoint (`bkpt`) and gives you access, within a
/// GDB session, to the stack frame (`_sf`) where the exception occurred.
// This needs asm!, #[naked] and unreachable() to avoid modifying the stack
// pointer (MSP), that way it points to the previous stack frame
#[naked]
pub unsafe extern "C" fn default_handler() {
    // This is the actual exception handler. `_sf` is a pointer to the previous
    // stack frame
    extern "C" fn handler(_sf: &StackFrame) -> ! {
        #[cfg(feature = "semihosting")]
        hprintln!("EXCEPTION {:?} @ PC=0x{:08x}", Exception::current(), _sf.pc);

        unsafe {
            bkpt!();
        }

        loop {}
    }

    // "trampoline" to get to the real exception handler.
    asm!("mrs r0, MSP
          ldr r1, [r0, #20]
          b $0"
         :
         : "i"(handler as extern "C" fn(&StackFrame) -> !) :: "volatile");

    ::core::intrinsics::unreachable()
}
