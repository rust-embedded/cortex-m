//! Interrupts

#[cfg(cortex_m)]
use core::arch::asm;
#[cfg(cortex_m)]
use core::sync::atomic::{compiler_fence, Ordering};

/// Trait for enums of external interrupt numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available external interrupts for a specific device.
/// Each variant must convert to a u16 of its interrupt number,
/// which is its exception number - 16.
///
/// # Safety
///
/// This trait must only be implemented on enums of device interrupts. Each
/// enum variant must represent a distinct value (no duplicates are permitted),
/// and must always return the same value (do not change at runtime).
///
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait InterruptNumber: Copy {
    /// Return the interrupt number associated with this variant.
    ///
    /// See trait documentation for safety requirements.
    fn number(self) -> u16;
}

/// Disables all interrupts in the current core.
#[cfg(cortex_m)]
#[inline]
pub fn disable() {
    unsafe {
        asm!("cpsid i", options(nomem, nostack, preserves_flags));
    }

    // Ensure no subsequent memory accesses are reordered to before interrupts are disabled.
    compiler_fence(Ordering::SeqCst);
}

/// Enables all the interrupts in the current core.
///
/// # Safety
///
/// - Do not call this function inside a critical section.
#[cfg(cortex_m)]
#[inline]
pub unsafe fn enable() {
    // Ensure no preceeding memory accesses are reordered to after interrupts are enabled.
    compiler_fence(Ordering::SeqCst);

    asm!("cpsie i", options(nomem, nostack, preserves_flags));
}

/// Execute closure `f` with interrupts disabled in the current core.
///
/// This method does not synchronize multiple cores and may disable required
/// interrupts on some platforms; see the `critical-section` crate for a cross-platform
/// way to enter a critical section which provides a `CriticalSection` token.
///
/// This crate provides an implementation for `critical-section` suitable for single-core systems,
/// based on disabling all interrupts. It can be enabled with the `critical-section-single-core` feature.
#[cfg(cortex_m)]
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // Backup previous state of PRIMASK register. We access the entire register directly as a
    // u32 instead of using the primask::read() function to minimize the number of processor
    // cycles during which interrupts are disabled.
    let primask = crate::register::primask::read_raw();

    // disable interrupts
    disable();

    let r = f();

    unsafe {
        crate::register::primask::write_raw(primask);
    }

    r
}

// Make a `free()` function available to allow checking dependencies without specifying a target,
// but that will panic at runtime if executed.
#[doc(hidden)]
#[cfg(not(cortex_m))]
#[inline]
pub fn free<F, R>(_: F) -> R
where
    F: FnOnce() -> R,
{
    panic!("cortex_m::interrupt::free() is only functional on cortex-m platforms");
}
