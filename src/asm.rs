//! Miscellaneous assembly instructions

/// Puts the processor in Debug state. Debuggers can pick this up as a "breakpoint".
///
/// **NOTE** calling `bkpt` when the processor is not connected to a debugger will cause an
/// exception.
#[inline(always)]
pub fn bkpt() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("bkpt" :::: "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __bkpt();
            }

            __bkpt();
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Blocks the program for *at least* `n` instruction cycles
///
/// This is implemented in assembly so its execution time is the same regardless of the optimization
/// level.
///
/// NOTE that the delay can take much longer if interrupts are serviced during its execution.
#[inline]
pub fn delay(_n: u32) {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe {
            asm!("1:
                  nop
                  subs $0, $$1
                  bne.n 1b"
                 : "+r"(_n / 4 + 1)
                 :
                 :
                 : "volatile");
        },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __delay(n: u32);
            }

            __delay(_n / 4 + 1);
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// A no-operation. Useful to prevent delay loops from being optimized away.
#[inline]
pub fn nop() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("nop" :::: "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __nop();
            }

            __nop()
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Wait For Event
#[inline]
pub fn wfe() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("wfe" :::: "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __wfe();
            }

            __wfe()
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Wait For Interrupt
#[inline]
pub fn wfi() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("wfi" :::: "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __wfi();
            }

            __wfi()
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Send Event
#[inline]
pub fn sev() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("sev" :::: "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __sev();
            }

            __sev()
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Instruction Synchronization Barrier
///
/// Flushes the pipeline in the processor, so that all instructions following the `ISB` are fetched
/// from cache or memory, after the instruction has been completed.
#[inline]
pub fn isb() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("isb 0xF" ::: "memory" : "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __isb();
            }

            __isb()
            // XXX do we need a explicit compiler barrier here?
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Data Synchronization Barrier
///
/// Acts as a special kind of memory barrier. No instruction in program order after this instruction
/// can execute until this instruction completes. This instruction completes only when both:
///
///  * any explicit memory access made before this instruction is complete
///  * all cache and branch predictor maintenance operations before this instruction complete
#[inline]
pub fn dsb() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("dsb 0xF" ::: "memory" : "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __dsb();
            }

            __dsb()
            // XXX do we need a explicit compiler barrier here?
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Data Memory Barrier
///
/// Ensures that all explicit memory accesses that appear in program order before the `DMB`
/// instruction are observed before any explicit memory accesses that appear in program order
/// after the `DMB` instruction.
#[inline]
pub fn dmb() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("dmb 0xF" ::: "memory" : "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __dmb();
            }

            __dmb()
            // XXX do we need a explicit compiler barrier here?
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
