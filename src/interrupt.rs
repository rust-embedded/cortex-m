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
    /// Borrows the data for the duration of the critical section
    pub fn borrow<'cs>(&self, _ctxt: &'cs CriticalSection) -> &'cs T {
        unsafe { &*self.inner.get() }
    }
}

/// Interrupt number
pub unsafe trait Nr {
    /// Returns the number associated with this interrupt
    fn nr(&self) -> u8;
}

unsafe impl<T> Sync for Mutex<T> {}

/// Disables all interrupts
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

/// Enables all the interrupts
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

/// Critical section context
///
/// Indicates that you are executing code within a critical section
pub struct CriticalSection {
    _0: (),
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    let primask = ::register::primask::read();

    // disable interrupts
    disable();

    let r = f(CriticalSection { _0: () });

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    if primask.is_active() {
        enable();
    }

    r
}
