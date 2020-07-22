//! Interrupts

pub use bare_metal::{CriticalSection, Mutex};

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
#[inline]
pub fn disable() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe {
            llvm_asm!("cpsid i" ::: "memory" : "volatile");
        },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __cpsid();
            }

            // XXX do we need a explicit compiler barrier here?
            __cpsid();
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn enable() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => llvm_asm!("cpsie i" ::: "memory" : "volatile"),

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => {
            extern "C" {
                fn __cpsie();
            }

            // XXX do we need a explicit compiler barrier here?
            __cpsie();
        }

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    let primask = crate::register::primask::read();

    // disable interrupts
    disable();

    let r = f(unsafe { &CriticalSection::new() });

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    if primask.is_active() {
        unsafe { enable() }
    }

    r
}
