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
#[inline(always)]
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
#[inline(always)]
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
#[inline(always)]
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

/// Instruction Synchronization Barrier
///
/// Flushes the pipeline in the processor, so that all instructions following the `ISB` are fetched
/// from cache or memory, after the instruction has been completed.
#[inline(always)]
pub fn isb() {
    match () {
        #[cfg(target_arch = "arm")]
        () => unsafe {
            asm!("isb 0xF" : : : "memory" : "volatile");
        },
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}

/// Data Synchronization Barrier
///
/// Acts as a special kind of memory barrier. No instruction in program order after this
/// instruction can execute until this instruction completes. This instruction completes only when
/// both:
///
///  * any explicit memory access made before this instruction is complete
///  * all cache and branch predictor maintenance operations before this instruction complete
#[inline(always)]
pub fn dsb() {
    match () {
        #[cfg(target_arch = "arm")]
        () => unsafe {
            asm!("dsb 0xF" : : : "memory" : "volatile");
        },
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}

/// Data Memory Barrier
///
/// Ensures that all explicit memory accesses that appear in program order before the `DMB`
/// instruction are observed before any explicit memory accesses that appear in program order
/// after the `DMB` instruction.
#[inline(always)]
pub fn dmb() {
    match () {
        #[cfg(target_arch = "arm")]
        () => unsafe {
            asm!("dmb 0xF" : : : "memory" : "volatile");
        },
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}
