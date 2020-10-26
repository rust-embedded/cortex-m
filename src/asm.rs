//! Miscellaneous assembly instructions

/// Puts the processor in Debug state. Debuggers can pick this up as a "breakpoint".
///
/// **NOTE** calling `bkpt` when the processor is not connected to a debugger will cause an
/// exception.
#[inline(always)]
pub fn bkpt() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { llvm_asm!("bkpt" :::: "volatile") },

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
    // NOTE(divide by 4) is easier to compute than `/ 3` is it's just a shift (`>> 2`).
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe {
            llvm_asm!("1:
                  nop
                  subs $0, #1
                  bne.n 1b"
                 : "+r"(_n / 4 + 1)
                 :
                 : "cpsr"
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
        () => unsafe { llvm_asm!("nop" :::: "volatile") },

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

/// Generate an Undefined Instruction exception.
///
/// Can be used as a stable alternative to `core::intrinsics::abort`.
#[inline]
pub fn udf() -> ! {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe {
            llvm_asm!("udf" :::: "volatile");
            core::hint::unreachable_unchecked();
        },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __udf();
            }

            __udf();

            core::hint::unreachable_unchecked();
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
        () => unsafe { llvm_asm!("wfe" :::: "volatile") },

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
        () => unsafe { llvm_asm!("wfi" :::: "volatile") },

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
        () => unsafe { llvm_asm!("sev" :::: "volatile") },

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
        () => unsafe { llvm_asm!("isb 0xF" ::: "memory" : "volatile") },

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
        () => unsafe { llvm_asm!("dsb 0xF" ::: "memory" : "volatile") },

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
        () => unsafe { llvm_asm!("dmb 0xF" ::: "memory" : "volatile") },

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

/// Test Target
///
/// Queries the Security state and access permissions of a memory location.
/// Returns a Test Target Response Payload (cf section D1.2.215 of
/// Armv8-M Architecture Reference Manual).
#[inline]
#[cfg(armv8m)]
// The __tt function does not dereference the pointer received.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn tt(addr: *mut u32) -> u32 {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => {
            let tt_resp: u32;
            unsafe {
                llvm_asm!("tt $0, $1" : "=r"(tt_resp) : "r"(addr) :: "volatile");
            }
            tt_resp
        }

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __tt(_: *mut u32) -> u32;
            }

            __tt(addr)
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Test Target Unprivileged
///
/// Queries the Security state and access permissions of a memory location for an unprivileged
/// access to that location.
/// Returns a Test Target Response Payload (cf section D1.2.215 of
/// Armv8-M Architecture Reference Manual).
#[inline]
#[cfg(armv8m)]
// The __ttt function does not dereference the pointer received.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn ttt(addr: *mut u32) -> u32 {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => {
            let tt_resp: u32;
            unsafe {
                llvm_asm!("ttt $0, $1" : "=r"(tt_resp) : "r"(addr) :: "volatile");
            }
            tt_resp
        }

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __ttt(_: *mut u32) -> u32;
            }

            __ttt(addr)
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Test Target Alternate Domain
///
/// Queries the Security state and access permissions of a memory location for a Non-Secure access
/// to that location. This instruction is only valid when executing in Secure state and is
/// undefined if used from Non-Secure state.
/// Returns a Test Target Response Payload (cf section D1.2.215 of
/// Armv8-M Architecture Reference Manual).
#[inline]
#[cfg(armv8m)]
// The __tta function does not dereference the pointer received.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn tta(addr: *mut u32) -> u32 {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => {
            let tt_resp: u32;
            unsafe {
                llvm_asm!("tta $0, $1" : "=r"(tt_resp) : "r"(addr) :: "volatile");
            }
            tt_resp
        }

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __tta(_: *mut u32) -> u32;
            }

            __tta(addr)
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Test Target Alternate Domain Unprivileged
///
/// Queries the Security state and access permissions of a memory location for a Non-Secure and
/// unprivileged access to that location. This instruction is only valid when executing in Secure
/// state and is undefined if used from Non-Secure state.
/// Returns a Test Target Response Payload (cf section D1.2.215 of
/// Armv8-M Architecture Reference Manual).
#[inline]
#[cfg(armv8m)]
// The __ttat function does not dereference the pointer received.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn ttat(addr: *mut u32) -> u32 {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => {
            let tt_resp: u32;
            unsafe {
                llvm_asm!("ttat $0, $1" : "=r"(tt_resp) : "r"(addr) :: "volatile");
            }
            tt_resp
        }

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __ttat(_: *mut u32) -> u32;
            }

            __ttat(addr)
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
