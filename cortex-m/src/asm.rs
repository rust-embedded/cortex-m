//! Miscellaneous assembly instructions

#![allow(missing_docs)]

#[cfg_attr(cortex_m, path = "asm/inner.rs")]
#[cfg_attr(not(cortex_m), path = "asm/inner_mock.rs")]
pub mod inner;

/// Puts the processor in Debug state. Debuggers can pick this up as a "breakpoint".
///
/// **NOTE** calling `bkpt` when the processor is not connected to a debugger will cause an
/// exception.
#[inline(always)]
pub fn bkpt() {
    unsafe { inner::__bkpt() };
}

/// Blocks the program for *at least* `cycles` CPU cycles.
///
/// This is implemented in assembly as a fixed number of iterations of a loop, so that execution
/// time is independent of the optimization level.
///
/// The loop code is the same for all architectures, however the number of CPU cycles required for
/// one iteration varies substantially between architectures.  This means that with a 48MHz CPU
/// clock, a call to `delay(48_000_000)` is guaranteed to take at least 1 second, but for example
/// could take 2 seconds.
///
/// NOTE that the delay can take much longer if interrupts are serviced during its execution and the
/// execution time may vary with other factors. This delay is mainly useful for simple timer-less
/// initialization of peripherals if and only if accurate timing is not essential. In any other case
/// please use a more accurate method to produce a delay.
#[inline]
pub fn delay(cycles: u32) {
    unsafe { inner::__delay(cycles) };
}

/// A no-operation. Useful to prevent delay loops from being optimized away.
#[inline]
pub fn nop() {
    unsafe { inner::__nop() };
}

/// Generate an Undefined Instruction exception.
///
/// Can be used as a stable alternative to `core::intrinsics::abort`.
#[inline]
pub fn udf() -> ! {
    unsafe { inner::__udf() }
}

/// Wait For Event
#[inline]
pub fn wfe() {
    unsafe { inner::__wfe() }
}

/// Wait For Interrupt
#[inline]
pub fn wfi() {
    unsafe { inner::__wfi() }
}

/// Send Event
#[inline]
pub fn sev() {
    unsafe { inner::__sev() }
}

/// Instruction Synchronization Barrier
///
/// Flushes the pipeline in the processor, so that all instructions following the `ISB` are fetched
/// from cache or memory, after the instruction has been completed.
#[inline]
pub fn isb() {
    unsafe { inner::__isb() }
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
    unsafe { inner::__dsb() }
}

/// Data Memory Barrier
///
/// Ensures that all explicit memory accesses that appear in program order before the `DMB`
/// instruction are observed before any explicit memory accesses that appear in program order
/// after the `DMB` instruction.
#[inline]
pub fn dmb() {
    unsafe { inner::__dmb() }
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
    let addr = addr as u32;
    unsafe { crate::asm::inner::__tt(addr) }
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
    let addr = addr as u32;
    unsafe { crate::asm::inner::__ttt(addr) }
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
    let addr = addr as u32;
    unsafe { crate::asm::inner::__tta(addr) }
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
    let addr = addr as u32;
    unsafe { crate::asm::inner::__ttat(addr) }
}

/// Branch and Exchange Non-secure
///
/// See section C2.4.26 of Armv8-M Architecture Reference Manual for details.
/// Undefined if executed in Non-Secure state.
#[inline]
#[cfg(armv8m)]
pub unsafe fn bx_ns(addr: u32) {
    unsafe { crate::asm::inner::__bxns(addr) };
}

/// Semihosting syscall.
///
/// This method is used by cortex-m-semihosting to provide semihosting syscalls.
#[inline]
pub unsafe fn semihosting_syscall(nr: u32, arg: u32) -> u32 {
    unsafe { inner::__sh_syscall(nr, arg) }
}

/// Switch to unprivileged mode using the Process Stack
///
/// Sets CONTROL.SPSEL (setting the Process Stack to be the active
/// stack) and CONTROL.nPRIV (setting unprivileged mode), updates the
/// program stack pointer to the address in `psp`, then jumps to the
/// address in `entry`.
///
/// # Safety
///
/// * `psp` and `entry` must point to valid stack memory and executable code,
///   respectively.
/// * `psp` must be 8 bytes aligned and point to stack top as stack grows
///   towards lower addresses.
/// * The size of the stack provided here must be large enough for your
///   program - stack overflows are obviously UB. If your processor supports
///   it, you may wish to set the `PSPLIM` register to guard against this.
#[cfg(cortex_m)]
#[inline(always)]
pub unsafe fn enter_unprivileged_psp(psp: *const u32, entry: extern "C" fn() -> !) -> ! {
    use crate::register::control::{Control, Npriv, Spsel};
    const CONTROL_FLAGS: u32 = {
        Control::from_bits(0)
            .with_npriv(Npriv::Unprivileged)
            .with_spsel(Spsel::Psp)
            .bits()
    };
    unsafe {
        core::arch::asm!(
            "msr     PSP, {psp}",
            "mrs     {tmp}, CONTROL",
            "orrs    {tmp}, {flags}",
            "msr     CONTROL, {tmp}",
            "isb",
            "bx      {ent}",
            tmp = in(reg) 0,
            flags = in(reg) CONTROL_FLAGS,
            psp = in(reg) psp,
            ent = in(reg) entry,
            options(noreturn, nostack)
        );
    }
}

/// Switch to using the Process Stack, but remain in Privileged Mode
///
/// Sets CONTROL.SPSEL (setting the Process Stack to be the active stack) but
/// leaves CONTROL.nPRIV alone, updates the program stack pointer to the
/// address in `psp`, then jumps to the address in `entry`.
///
/// # Safety
///
/// * `psp` and `entry` must point to valid stack memory and executable code,
///   respectively.
/// * `psp` must be 8 bytes aligned and point to stack top as stack grows
///   towards lower addresses.
/// * The size of the stack provided here must be large enough for your
///   program - stack overflows are obviously UB. If your processor supports
///   it, you may wish to set the `PSPLIM` register to guard against this.
#[cfg(cortex_m)]
#[inline(always)]
pub unsafe fn enter_privileged_psp(psp: *const u32, entry: extern "C" fn() -> !) -> ! {
    use crate::register::control::{Control, Npriv, Spsel};
    const CONTROL_FLAGS: u32 = {
        Control::from_bits(0)
            .with_npriv(Npriv::Privileged)
            .with_spsel(Spsel::Psp)
            .bits()
    };
    unsafe {
        core::arch::asm!(
            "msr     PSP, {psp}",
            "mrs     {tmp}, CONTROL",
            "orrs    {tmp}, {flags}",
            "msr     CONTROL, {tmp}",
            "isb",
            "bx      {ent}",
            tmp = in(reg) 0,
            flags = in(reg) CONTROL_FLAGS,
            psp = in(reg) psp,
            ent = in(reg) entry,
            options(noreturn, nostack)
        );
    }
}

/// Bootstrap.
///
/// Clears CONTROL.SPSEL (setting the main stack to be the active stack),
/// updates the main stack pointer to the address in `msp`, then jumps
/// to the address in `rv`.
///
/// # Safety
///
/// `msp` and `rv` must point to valid stack memory and executable code,
/// respectively.
#[inline]
pub unsafe fn bootstrap(msp: *const u32, rv: *const u32) -> ! {
    // Ensure thumb mode is set.
    let rv = (rv as u32) | 1;
    let msp = msp as u32;
    unsafe { inner::__bootstrap(msp, rv) }
}

/// Bootload.
///
/// Reads the initial stack pointer value and reset vector from
/// the provided vector table address, sets the active stack to
/// the main stack, sets the main stack pointer to the new initial
/// stack pointer, then jumps to the reset vector.
///
/// # Safety
///
/// The provided `vector_table` must point to a valid vector
/// table, with a valid stack pointer as the first word and
/// a valid reset vector as the second word.
#[inline]
pub unsafe fn bootload(vector_table: *const u32) -> ! {
    unsafe {
        let msp = core::ptr::read_volatile(vector_table);
        let rv = core::ptr::read_volatile(vector_table.offset(1));
        bootstrap(msp as *const u32, rv as *const u32);
    }
}
