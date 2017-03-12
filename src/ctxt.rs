//! Interrupt / Exception context local data
//!
//! The main use case is safely adding state to exception / interrupt handlers.
//!
//! This is done in two stages, first you define a token that will appear in the
//! interrupt handler signature; each handler will have its unique token. This
//! token must be zero sized type because interrupt handlers' real signature is
//! `fn()` and it must also implement the `Context` trait. You must also make
//! sure that the token can't be constructed outside of the crate where it's
//! defined.
//!
//! ```
//! # use cortex_m::ctxt::Context;
//! // This must be in a library crate
//! /// Token unique to the TIM7 interrupt handler
//! pub struct Tim7 { _0: () }
//!
//! unsafe impl Context for Tim7 {}
//! ```
//!
//! Then in the application one can pin data to the interrupt handler using
//! `Local`.
//!
//! ```
//! # #![feature(const_fn)]
//! # use std::cell::Cell;
//! # use cortex_m::ctxt::{Context, Local};
//! # struct Tim7;
//! # unsafe impl Context for Tim7 {}
//! // omitted: how to put this handler in the vector table
//! extern "C" fn tim7(ctxt: Tim7) {
//!     static STATE: Local<Cell<bool>, Tim7> = Local::new(Cell::new(false));
//!
//!     let state = STATE.borrow(&ctxt);
//!
//!     // toggle state
//!     state.set(!state.get());
//!
//!     if state.get() {
//!         // something
//!     } else {
//!         // something else
//!     }
//! }
//! ```
//!
//! Note that due to the uniqueness of tokens, other handlers won't be able to
//! access context local data. (Given that you got the signatures right)
//!
//! ```
//! # #![feature(const_fn)]
//! # use std::cell::Cell;
//! # use cortex_m::ctxt::{Context, Local};
//! # struct Tim3;
//! # struct Tim4;
//! static TIM3_DATA: Local<Cell<bool>, Tim3> = Local::new(Cell::new(false));
//!
//! extern "C" fn tim3(ctxt: Tim3) {
//!     let data = TIM3_DATA.borrow(&ctxt);
//! }
//!
//! extern "C" fn tim4(ctxt: Tim4) {
//!     //let data = TIM3_DATA.borrow(&ctxt);
//!     // ^ wouldn't work
//! }
//! # unsafe impl Context for Tim3 {}
//! # fn main() {}
//! ```
//!
//! To have the application use these tokenized function signatures, you can
//! define, in a library, a `Handlers` struct that represents the vector table:
//!
//! ```
//! # struct Tim1;
//! # struct Tim2;
//! # struct Tim3;
//! # struct Tim4;
//! # extern "C" fn default_handler<T>(_: T) {}
//! #[repr(C)]
//! pub struct Handlers {
//!     tim1: extern "C" fn(Tim1),
//!     tim2: extern "C" fn(Tim2),
//!     tim3: extern "C" fn(Tim3),
//!     tim4: extern "C" fn(Tim4),
//!     /* .. */
//! }
//!
//! pub const DEFAULT_HANDLERS: Handlers = Handlers {
//!     tim1: default_handler,
//!     tim2: default_handler,
//!     tim3: default_handler,
//!     tim4: default_handler,
//!     /* .. */
//! };
//! ```
//!
//! Then have the user use that `struct` to register the interrupt handlers:
//!
//! ```
//! # struct Tim3;
//! # struct Handlers { tim3: extern "C" fn(Tim3), tim4: extern "C" fn(Tim3) }
//! # const DEFAULT_HANDLERS: Handlers = Handlers { tim3: tim3, tim4: tim3 };
//! extern "C" fn tim3(ctxt: Tim3) { /* .. */ }
//!
//! // override the TIM3 interrupt handler
//! #[no_mangle]
//! static _INTERRUPTS: Handlers = Handlers {
//!     tim3: tim3, ..DEFAULT_HANDLERS
//! };
//! ```
//!
//! This pattern is implemented for exceptions in this crate. See
//! `exception::Handlers` and `exception::DEFAULT_HANDLERS`.

use core::marker::PhantomData;
use core::cell::UnsafeCell;

/// Data local to a context
pub struct Local<T, Ctxt>
where
    Ctxt: Context,
{
    _ctxt: PhantomData<Ctxt>,
    data: UnsafeCell<T>,
}

impl<T, Ctxt> Local<T, Ctxt>
where
    Ctxt: Context,
{
    /// Initializes context local data
    pub const fn new(value: T) -> Self {
        Local {
            _ctxt: PhantomData,
            data: UnsafeCell::new(value),
        }
    }

    /// Acquires a reference to the context local data
    pub fn borrow<'ctxt>(&'static self, _ctxt: &'ctxt Ctxt) -> &'ctxt T {
        unsafe { &*self.data.get() }
    }

    /// Acquires a mutable reference to the context local data
    pub fn borrow_mut<'ctxt>(
        &'static self,
        _ctxt: &'ctxt mut Ctxt,
    ) -> &'ctxt mut T {
        unsafe { &mut *self.data.get() }
    }
}

unsafe impl<T, Ctxt> Sync for Local<T, Ctxt>
where
    Ctxt: Context,
{
}

/// A token unique to a context
pub unsafe trait Context {}
