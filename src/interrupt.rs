//! Interrupts

// use core::sync::atomic::{self, Ordering};

pub use bare_metal::{CriticalSection, Mutex, Nr};

/// Disables all interrupts
#[inline]
pub fn disable() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe {
            asm!("cpsid i" ::: "memory" : "volatile");
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
        () => asm!("cpsie i" ::: "memory" : "volatile"),

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
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    let primask = ::register::primask::read();

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
