//! Miscellaneous assembly instructions

#![allow(missing_docs)]

#[cfg(cortex_m)]
use core::arch::asm;
#[cfg(cortex_m)]
use core::sync::atomic::{Ordering, compiler_fence};
use cortex_m_macros::asm_cfg;

/// Puts the processor in Debug state. Debuggers can pick this up as a "breakpoint".
///
/// **NOTE** calling `bkpt` when the processor is not connected to a debugger will cause an
/// exception.
#[inline(always)]
#[asm_cfg(cortex_m)]
pub fn bkpt() {
    unsafe { asm!("bkpt", options(nomem, nostack, preserves_flags)) };
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
#[asm_cfg(cortex_m)]
pub fn delay(cycles: u32) {
    // The loop will normally take 3 to 4 CPU cycles per iteration, but superscalar cores
    // (eg. Cortex-M7) can potentially do it in 2, so we use that as the lower bound, since delaying
    // for more cycles is okay.
    // Add 1 to prevent an integer underflow which would cause a long freeze
    let real_cyc = 1 + cycles / 2;
    unsafe {
        asm!(
            // The `bne` on some cores (eg Cortex-M4) will take a different number of instructions
            // depending on the alignment of the branch target.  Set the alignment of the top of the
            // loop to prevent surprising timing changes when the alignment of the delay() changes.
            ".p2align 3",
            // Use local labels to avoid R_ARM_THM_JUMP8 relocations which fail on thumbv6m.
            "2:", // not 1 or 0 because of https://github.com/llvm/llvm-project/issues/99547
            "subs {}, #1", // subtract 1 from real_cyc
            "bne 2b",      // branch to 2 if result is non-zero
            inout(reg) real_cyc => _,
            options(nomem, nostack),
        )
    };
}

/// A no-operation. Useful to prevent delay loops from being optimized away.
#[inline]
#[asm_cfg(cortex_m)]
pub fn nop() {
    // NOTE: This is a `pure` asm block, but applying that option allows the compiler to eliminate
    // the nop entirely (or to collapse multiple subsequent ones). Since the user probably wants N
    // nops when they call `nop` N times, let's not add that option.
    unsafe { asm!("nop", options(nomem, nostack, preserves_flags)) };
}

/// Generate an Undefined Instruction exception.
///
/// Can be used as a stable alternative to `core::intrinsics::abort`.
#[inline]
#[asm_cfg(cortex_m)]
pub fn udf() -> ! {
    unsafe { asm!("udf #0", options(noreturn, nomem, nostack, preserves_flags)) };
}

/// Wait For Event
#[inline]
#[asm_cfg(cortex_m)]
pub fn wfe() {
    unsafe { asm!("wfe", options(nomem, nostack, preserves_flags)) };
}

/// Wait For Interrupt
#[inline]
#[asm_cfg(cortex_m)]
pub fn wfi() {
    unsafe { asm!("wfi", options(nomem, nostack, preserves_flags)) };
}

/// Send Event
#[inline]
#[asm_cfg(cortex_m)]
pub fn sev() {
    unsafe { asm!("sev", options(nomem, nostack, preserves_flags)) };
}

/// Instruction Synchronization Barrier
///
/// Flushes the pipeline in the processor, so that all instructions following the `ISB` are fetched
/// from cache or memory, after the instruction has been completed.
#[inline]
#[asm_cfg(cortex_m)]
pub fn isb() {
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("isb", options(nostack, preserves_flags)) };
    compiler_fence(Ordering::SeqCst);
}

/// Data Synchronization Barrier
///
/// Acts as a special kind of memory barrier. No instruction in program order after this instruction
/// can execute until this instruction completes. This instruction completes only when both:
///
///  * any explicit memory access made before this instruction is complete
///  * all cache and branch predictor maintenance operations before this instruction complete
#[inline]
#[asm_cfg(cortex_m)]
pub fn dsb() {
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("dsb", options(nostack, preserves_flags)) };
    compiler_fence(Ordering::SeqCst);
}

/// Data Memory Barrier
///
/// Ensures that all explicit memory accesses that appear in program order before the `DMB`
/// instruction are observed before any explicit memory accesses that appear in program order
/// after the `DMB` instruction.
#[inline]
#[asm_cfg(cortex_m)]
pub fn dmb() {
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("dmb", options(nostack, preserves_flags)) };
    compiler_fence(Ordering::SeqCst);
}

/// Test Target
///
/// Queries the Security state and access permissions of a memory location.
/// Returns a Test Target Response Payload (cf section D1.2.215 of
/// Armv8-M Architecture Reference Manual).
#[inline]
#[asm_cfg(armv8m)]
// The __tt function does not dereference the pointer received.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn tt(addr: *mut u32) -> u32 {
    let mut addr = addr as u32;
    unsafe {
        asm!(
            "tt {addr}, {addr}",
            addr = inout(reg) addr,
            options(nomem, nostack, preserves_flags),
        )
    };
    addr
}

/// Test Target Unprivileged
///
/// Queries the Security state and access permissions of a memory location for an unprivileged
/// access to that location.
/// Returns a Test Target Response Payload (cf section D1.2.215 of
/// Armv8-M Architecture Reference Manual).
#[inline]
#[asm_cfg(armv8m)]
// The __ttt function does not dereference the pointer received.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn ttt(addr: *mut u32) -> u32 {
    let mut addr = addr as u32;
    unsafe {
        asm!(
            "ttt {addr}, {addr}",
            addr = inout(reg)addr,
            options(nomem, nostack, preserves_flags),
        )
    };
    addr
}

/// Test Target Alternate Domain
///
/// Queries the Security state and access permissions of a memory location for a Non-Secure access
/// to that location. This instruction is only valid when executing in Secure state and is
/// undefined if used from Non-Secure state.
/// Returns a Test Target Response Payload (cf section D1.2.215 of
/// Armv8-M Architecture Reference Manual).
#[inline]
#[asm_cfg(armv8m)]
// The __tta function does not dereference the pointer received.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn tta(addr: *mut u32) -> u32 {
    let mut addr = addr as u32;
    unsafe {
        asm!(
            "tta {addr}, {addr}",
            addr = inout(reg) addr,
            options(nomem, nostack, preserves_flags),
        )
    };
    addr
}

/// Test Target Alternate Domain Unprivileged
///
/// Queries the Security state and access permissions of a memory location for a Non-Secure and
/// unprivileged access to that location. This instruction is only valid when executing in Secure
/// state and is undefined if used from Non-Secure state.
/// Returns a Test Target Response Payload (cf section D1.2.215 of
/// Armv8-M Architecture Reference Manual).
#[inline]
#[asm_cfg(armv8m)]
// The __ttat function does not dereference the pointer received.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn ttat(addr: *mut u32) -> u32 {
    let mut addr = addr as u32;
    unsafe {
        asm!(
            "ttat {addr}, {addr}",
            addr = inout(reg) addr,
            options(nomem, nostack, preserves_flags),
        )
    };
    addr
}

/// Branch and Exchange Non-secure
///
/// See section C2.4.26 of Armv8-M Architecture Reference Manual for details.
/// Undefined if executed in Non-Secure state.
#[inline]
#[asm_cfg(armv8m)]
pub unsafe fn bx_ns(addr: u32) {
    unsafe { asm!("BXNS {}", in(reg) addr, options(nomem, nostack, preserves_flags)) };
}

/// Semihosting syscall.
///
/// This method is used by cortex-m-semihosting to provide semihosting syscalls.
#[inline]
#[asm_cfg(cortex_m)]
pub unsafe fn semihosting_syscall(mut nr: u32, arg: u32) -> u32 {
    unsafe {
        asm!("bkpt #0xab", inout("r0") nr, in("r1") arg, options(nomem, nostack, preserves_flags))
    };
    nr
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
#[inline(always)]
#[asm_cfg(cortex_m)]
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
#[inline(always)]
#[asm_cfg(cortex_m)]
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
#[asm_cfg(cortex_m)]
pub unsafe fn bootstrap(msp: *const u32, rv: *const u32) -> ! {
    // Ensure thumb mode is set.
    let rv = (rv as u32) | 1;
    let msp = msp as u32;
    unsafe {
        asm!(
            "mrs {tmp}, CONTROL",
            "bics {tmp}, {spsel}",
            "msr CONTROL, {tmp}",
            "isb",
            "msr MSP, {msp}",
            "bx {rv}",
            // `out(reg) _` is not permitted in a `noreturn` asm! call,
            // so instead use `in(reg) 0` and don't restore it afterwards.
            tmp = in(reg) 0,
            spsel = in(reg) 2,
            msp = in(reg) msp,
            rv = in(reg) rv,
            options(noreturn, nomem, nostack),
        )
    };
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
#[asm_cfg(cortex_m)]
pub unsafe fn bootload(vector_table: *const u32) -> ! {
    unsafe {
        let msp = core::ptr::read_volatile(vector_table);
        let rv = core::ptr::read_volatile(vector_table.offset(1));
        bootstrap(msp as *const u32, rv as *const u32);
    }
}
