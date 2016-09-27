//! Miscellaneous assembly instructions

/// Puts the processor in Debug state. Debuggers can pick this up as a "breakpoint".
///
/// Optionally, an "immediate" value (in the 0-255 range) can be passed to `bkpt!`. The debugger can
/// then read this value using the Program Counter (PC).
#[macro_export]
macro_rules! bkpt {
    () => {
        asm!("bkpt" :::: "volatile");
    };
    ($imm:expr) => {
        asm!(concat!("bkpt #", stringify!($imm)) :::: "volatile");
    };
}

/// Wait for event
pub unsafe fn wfe() {
    match () {
        #[cfg(target_arch = "arm")]
        () => asm!("wfe" :::: "volatile"),
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}

/// Wait for interupt
pub unsafe fn wfi() {
    match () {
        #[cfg(target_arch = "arm")]
        () => asm!("wfi" :::: "volatile"),
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}
