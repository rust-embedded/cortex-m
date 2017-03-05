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

/// Interrupt number
pub unsafe trait Nr {
    /// Returns the number associated with this interrupt
    fn nr(&self) -> u8;
}

// FIXME `T` should have some bound: `Send` or `Sync`?
unsafe impl<T> Sync for Mutex<T> {}

/// Disable interrupts, globally
#[inline(always)]
pub fn disable() {
    match () {
        #[cfg(target_arch = "arm")]
        () => unsafe {
            asm!("cpsid i"
                 :
                 :
                 :
                 : "volatile");
        },
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}

/// Enable interrupts, globally
#[inline(always)]
pub fn enable() {
    match () {
        #[cfg(target_arch = "arm")]
        () => unsafe {
            asm!("cpsie i"
                 :
                 :
                 :
                 : "volatile");
        },
        #[cfg(not(target_arch = "arm"))]
        () => {}
    }
}

/// Critical section token
///
/// Indicates that you are executing code within a critical section
pub struct CsToken {
    _private: (),
}

/// Execute closure `f` in an interrupt-free context.
/// This as also known as a "critical section".
pub fn free<F, R>(f: F) -> R
    where F: FnOnce(&CsToken) -> R
{
    let primask = ::register::primask::read();

    // disable interrupts
    disable();

    let r = f(&CsToken { _private: () });

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    if primask.is_active() {
        enable();
    }

    r
}
