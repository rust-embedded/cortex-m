//! Interrupts

#[cfg(cortex_m)]
use core::arch::asm;
#[cfg(cortex_m)]
use core::sync::atomic::{compiler_fence, Ordering};

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
/// This method does not synchronise multiple cores and may disable required
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
    let primask = crate::register::primask::read();

    // disable interrupts
    disable();

    let r = f();

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    if primask.is_active() {
        unsafe { enable() }
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
