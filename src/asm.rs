//! Miscellaneous assembly instructions

/// Puts the processor in Debug state. Debuggers can pick this up as a
/// "breakpoint".
///
/// NOTE calling `bkpt` when the processor is not connected to a debugger will
/// cause an exception
#[inline(always)]
pub fn bkpt() {
    #[cfg(target_arch = "arm")]
    unsafe {
        asm!("bkpt"
             :
             :
             :
             : "volatile");
    }
}

/// A no-operation. Useful to prevent delay loops from being optimized away.
pub fn nop() {
    unsafe {
        asm!("nop"
             :
             :
             :
             : "volatile");
    }
}
/// Wait For Event
pub fn wfe() {
    match () {
        #[cfg(target_arch = "arm")]
        () => unsafe {
            asm!("wfe"
                 :
                 :
                 :
                 : "volatile")
        },
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}

/// Wait For Interrupt
pub fn wfi() {
    match () {
        #[cfg(target_arch = "arm")]
        () => unsafe{
            asm!("wfi"
                 :
                 :
                 :
                 : "volatile")
        },
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}
