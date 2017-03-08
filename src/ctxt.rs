//! Execution context

use core::marker::PhantomData;
use core::cell::UnsafeCell;

/// Data local to an execution context
pub struct Local<T, Ctxt>
    where Ctxt: Token
{
    _ctxt: PhantomData<Ctxt>,
    data: UnsafeCell<T>,
}

impl<T, Ctxt> Local<T, Ctxt>
    where Ctxt: Token
{
    /// Initializes the local data
    pub const fn new(value: T) -> Self {
        Local {
            _ctxt: PhantomData,
            data: UnsafeCell::new(value),
        }
    }

    /// Acquires a reference to the local data
    pub fn borrow<'a>(&'static self, _ctxt: &'a Ctxt) -> &'a T {
        unsafe { &*self.data.get() }
    }
}

unsafe impl<T, Ctxt> Sync for Local<T, Ctxt> where Ctxt: Token {}

/// A unique token that identifies an execution context
pub unsafe trait Token {}
