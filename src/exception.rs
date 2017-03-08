//! Exceptions

use ctxt::Token;
use Reserved;

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
        match unsafe { (*::peripheral::SCB.get()).icsr.read() } as u8 {
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
    pub nmi: unsafe extern "C" fn(&NmiCtxt),
    /// All class of fault
    pub hard_fault: unsafe extern "C" fn(&HardFaultCtxt),
    /// Memory management
    pub mem_manage: unsafe extern "C" fn(&MemManageCtxt),
    /// Pre-fetch fault, memory access fault
    pub bus_fault: unsafe extern "C" fn(&BusFaultCtxt),
    /// Undefined instruction or illegal state
    pub usage_fault: unsafe extern "C" fn(&UsageFaultCtxt),
    /// Reserved spots in the vector table
    pub _reserved0: [Reserved; 4],
    /// System service call via SWI instruction
    pub svcall: unsafe extern "C" fn(&SvcallCtxt),
    /// Reserved spots in the vector table
    pub _reserved1: [Reserved; 2],
    /// Pendable request for system service
    pub pendsv: unsafe extern "C" fn(&PendsvCtxt),
    /// System tick timer
    pub sys_tick: unsafe extern "C" fn(&SysTickCtxt),
}

/// Identifies the Nmi exception
pub struct NmiCtxt {
    _0: (),
}

/// Identifies the HardFault exception
pub struct HardFaultCtxt {
    _0: (),
}

/// Identifies the MemManage exception
pub struct MemManageCtxt {
    _0: (),
}

/// Identifies the BusFault exception
pub struct BusFaultCtxt {
    _0: (),
}

/// Identifies the UsageFault exception
pub struct UsageFaultCtxt {
    _0: (),
}

/// Identifies the Svcall exception
pub struct SvcallCtxt {
    _0: (),
}

/// Identifies the Pendsv exception
pub struct PendsvCtxt {
    _0: (),
}

/// Identifies the Systick exception
pub struct SysTickCtxt {
    _0: (),
}

unsafe impl Token for NmiCtxt {}

unsafe impl Token for HardFaultCtxt {}

unsafe impl Token for MemManageCtxt {}

unsafe impl Token for BusFaultCtxt {}

unsafe impl Token for SvcallCtxt {}

unsafe impl Token for PendsvCtxt {}

unsafe impl Token for SysTickCtxt {}

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
// pointer (MSP), that way it points to the stacked registers
#[naked]
pub unsafe extern "C" fn default_handler<T>(_token: &T) {
    // This is the actual exception handler. `_sf` is a pointer to the previous
    // stack frame
    #[cfg(target_arch = "arm")]
    extern "C" fn handler(_sr: &StackedRegisters) -> ! {
        #[cfg(feature = "semihosting")]
        hprintln!("EXCEPTION {:?} @ PC=0x{:08x}", Exception::current(), _sr.pc);

        ::asm::bkpt();

        loop {}
    }

    match () {
        #[cfg(target_arch = "arm")]
        () => {
            // "trampoline" to get to the real exception handler.
            asm!("mrs r0, MSP
                  ldr r1, [r0, #20]
                  b $0"
                 :
                 : "i"(handler as extern "C" fn(&StackedRegisters) -> !)
                 :
                 : "volatile");

            ::core::intrinsics::unreachable()
        }
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}

/// Registers stacked during an exception
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
