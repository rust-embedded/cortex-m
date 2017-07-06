//! Interrupts

pub use bare_metal::{CriticalSection, Mutex, Nr};

/// Disables all interrupts
#[inline(always)]
pub fn disable() {
    match () {
        #[cfg(target_arch = "arm")]
        () => unsafe {
            asm!("cpsid i"
                 :
                 :
                 : "memory"
                 : "volatile");
        },
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}

/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline(always)]
pub unsafe fn enable() {
    match () {
        #[cfg(target_arch = "arm")]
        () => {
            asm!("cpsie i"
                 :
                 :
                 : "memory"
                 : "volatile");
        }
        #[cfg(not(target_arch = "arm"))]
        () => {}
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
