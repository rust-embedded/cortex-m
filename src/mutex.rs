//! Implementation of a critical section based mutex that also implements the `mutex-trait`.

use core::cell::RefCell;

/// A critical section based mutex
pub struct CriticalSectionMutex<T> {
    data: RefCell<T>,
}

impl<T> CriticalSectionMutex<T> {
    /// Create a new mutex
    pub const fn new(data: T) -> Self {
        CriticalSectionMutex {
            data: RefCell::new(data),
        }
    }
}

impl<T> mutex_trait::Mutex for &'_ CriticalSectionMutex<T> {
    type Data = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        crate::interrupt::free(|_| f(&mut *self.data.borrow_mut()))
    }
}
