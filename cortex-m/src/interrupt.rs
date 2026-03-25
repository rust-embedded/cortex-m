//! Interrupts

use core::arch::asm;
use core::sync::atomic::{Ordering, compiler_fence};

pub use bare_metal::{CriticalSection, Mutex, Nr};

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

/// Implement InterruptNumber for the old bare_metal::Nr trait.
/// This implementation is for backwards compatibility only and will be removed in cortex-m 0.8.
unsafe impl<T: Nr + Copy> InterruptNumber for T {
    #[inline]
    fn number(self) -> u16 {
        self.nr() as u16
    }
}

/// Disables all interrupts
#[inline]
pub fn disable() {
    unsafe { asm!("cpsid i", options(nomem, nostack, preserves_flags)) };

    // Ensure no subsequent memory accesses are reordered to before interrupts are disabled.
    compiler_fence(Ordering::SeqCst);
}

/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
#[cortex_m_macros::asm_cfg(any(armv6m, armv7m, armv7em, armv8m))]
pub unsafe fn enable() {
    // Ensure no preceeding memory accesses are reordered to after interrupts are enabled.
    compiler_fence(Ordering::SeqCst);

    unsafe { asm!("cpsie i", options(nomem, nostack, preserves_flags)) };
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[cfg(cortex_m)]
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    // Backup previous state of PRIMASK register. We access the entire register directly as a
    // u32 instead of using the primask::read() function to minimize the number of processor
    // cycles during which interrupts are disabled.
    let primask = crate::register::primask::read_raw();

    // disable interrupts
    disable();

    let r = f(&unsafe { CriticalSection::new() });

    unsafe {
        crate::register::primask::write_raw(primask);
    }

    r
}

// Make a `free()` function available on hosted platforms to allow checking dependencies without
// specifying a target, but that will panic at runtime if executed.
/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[cfg(not(cortex_m))]
#[inline]
pub fn free<F, R>(_: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    panic!("cortex_m::interrupt::free() is only functional on cortex-m platforms");
}
