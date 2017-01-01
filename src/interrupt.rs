//! Interrupts

use core::cell::UnsafeCell;

/// A "mutex" based on critical sections
pub struct Mutex<T> {
    inner: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    /// Creates a new mutex
    pub const fn new(value: T) -> Self {
        Mutex { inner: UnsafeCell::new(value) }
    }
}

impl<T> Mutex<T> {
    /// Gets access to the inner data
    ///
    /// NOTE this prevents interrupts handlers from running thus gaining
    /// exclusive access to the processor
    pub fn lock<F, R>(&self, f: F) -> R
        where F: FnOnce(&mut T) -> R
    {
        unsafe { ::interrupt::free(|_| f(&mut *self.inner.get())) }
    }
}

// FIXME `T` should have some bound: `Send` or `Sync`?
unsafe impl<T> Sync for Mutex<T> {}

/// A struct whos existence guarantees that interrupts are disabled
///
/// This struct is zero-sized and cannot be initialized by user code. An
/// instance is only ever created in the `free` function, which passes the
/// instance to the closure. This allows a user to force a function to only
/// ever be called in a critical section by taking a reference to a `CritToken`
/// as a parameter.
pub struct CritToken(());

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
pub unsafe fn free<F, R>(f: F) -> R
    where F: FnOnce(&CritToken) -> R
{
    let primask = ::register::primask::read();

    disable();

    let r = f(&CritToken(()));

    // If the interrupts were enabled before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    // PRIMASK & 1 = 1 indicates that the interrupts were disabled
    // PRIMASK & 1 = 0 indicates that they were enabled
    if primask & 1 == 0 {
        enable();
    }

    r
}
