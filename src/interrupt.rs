//! Interrupts

/// Disable interrupts, globally
#[inline(always)]
pub unsafe fn disable() {
    match () {
        #[cfg(target_arch = "arm")]
        () => {
            asm!("cpsid i" :::: "volatile");
        }
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}

/// Enable interrupts, globally
#[inline(always)]
pub unsafe fn enable() {
    match () {
        #[cfg(target_arch = "arm")]
        () => {
            asm!("cpsie i" :::: "volatile");
        }
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}

/// Execute closure `f` in an interrupt-free context.
/// This as also known as a "critical section".
pub unsafe fn free<F>(f: F)
    where F: FnOnce()
{
    let primask = ::register::primask::read();

    disable();

    f();

    // If the interrupts were enabled before our `disable` call, then re-enable them
    // Otherwise, keep them disabled
    // PRIMASK & 1 = 1 indicates that the interrupts were disabled
    // PRIMASK & 1 = 0 indicates that they were enabled
    if primask & 1 == 0 {
        enable();
    }
}
