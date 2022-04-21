//! Interrupts

pub use bare_metal::{CriticalSection, Mutex};
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

/// Disables all interrupts
#[cfg(cortex_m)]
#[inline]
pub fn disable() {
    unsafe {
        asm!("cpsid i", options(nomem, nostack, preserves_flags));
    }

    // Ensure no subsequent memory accesses are reordered to before interrupts are disabled.
    compiler_fence(Ordering::SeqCst);
}

/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[cfg(cortex_m)]
#[inline]
pub unsafe fn enable() {
    // Ensure no preceeding memory accesses are reordered to after interrupts are enabled.
    compiler_fence(Ordering::SeqCst);

    asm!("cpsie i", options(nomem, nostack, preserves_flags));
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[cfg(cortex_m)]
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection<'_>) -> R,
{
    let primask = crate::register::primask::read();

    // disable interrupts
    disable();

    let r = f(unsafe { CriticalSection::new() });

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
    F: FnOnce(CriticalSection<'_>) -> R,
{
    panic!("cortex_m::interrupt::free() is only functional on cortex-m platforms");
}
