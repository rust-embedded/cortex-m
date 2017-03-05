//! Miscellaneous assembly instructions

/// Puts the processor in Debug state. Debuggers can pick this up as a
/// "breakpoint".
///
/// Optionally, an "immediate" value (in the 0-255 range) can be passed to
/// `bkpt!`. The debugger can then read this value using the Program Counter
/// (PC).
#[cfg(target_arch = "arm")]
#[macro_export]
macro_rules! bkpt {
    () => {
        asm!("bkpt"
             :
             :
             :
             : "volatile");
    };
    ($imm:expr) => {
        asm!(concat!("bkpt #", stringify!($imm))
             :
             :
             :
             : "volatile");
    };
}

/// Puts the processor in Debug state. Debuggers can pick this up as a
/// "breakpoint".
///
/// Optionally, an "immediate" value (in the 0-255 range) can be passed to
/// `bkpt!`. The debugger can then read this value using the Program Counter
/// (PC).
#[cfg(not(target_arch = "arm"))]
#[macro_export]
macro_rules! bkpt {
    () => {
        asm!("");
    };
    ($e:expr) => {
        asm!("");
    };
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
