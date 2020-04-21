//! Implementation of a critical section based mutex that also implements the `mutex-trait`.

use core::cell::UnsafeCell;

/// A critical section based mutex
pub struct CriticalSectionMutex<T> {
    data: UnsafeCell<T>,
}

impl<T> CriticalSectionMutex<T> {
    /// Create a new mutex
    pub const fn new(data: T) -> Self {
        CriticalSectionMutex {
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> mutex_trait::Mutex for &'_ CriticalSectionMutex<T> {
    type Data = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        crate::interrupt::free(|_| f(unsafe { &mut *self.data.get() }))
    }
}

// NOTE A `Mutex` can be used as a channel so the protected data must be `Send`
// to prevent sending non-Sendable stuff (e.g. access tokens) across different
// execution contexts (e.g. interrupts)
unsafe impl<T> Sync for CriticalSectionMutex<T> where T: Send {}
