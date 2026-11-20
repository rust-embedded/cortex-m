//! Interrupts

#[cfg(not(feature = "bare-metal"))]
pub use self::compat::{CriticalSection, Mutex, Nr};
#[cfg(feature = "bare-metal")]
pub use bare_metal::{CriticalSection, Mutex, Nr};
#[cfg(cortex_m)]
use core::arch::asm;
#[cfg(cortex_m)]
use core::sync::atomic::{Ordering, compiler_fence};
use cortex_m_macros::asm_cfg;

/// Trait for enums of external interrupt numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available external interrupts for a specific device.
/// Each variant must convert to a u16 of its interrupt number,
/// which is its exception number - 16.
///
/// # Safety
///
/// This trait must only be implemented on enums of device interrupts. Each
/// enum variant must represent a distinct value (no duplicates are permitted),
/// and must always return the same value (do not change at runtime).
///
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait InterruptNumber: Copy {
    /// Return the interrupt number associated with this variant.
    ///
    /// See trait documentation for safety requirements.
    fn number(self) -> u16;
}

/// Implement InterruptNumber for the old `Nr` trait (`bare_metal::Nr` with the
/// `bare-metal` feature).
/// This implementation is for backwards compatibility only and will be removed in cortex-m 0.8.
unsafe impl<T: Nr + Copy> InterruptNumber for T {
    #[inline]
    fn number(self) -> u16 {
        self.nr() as u16
    }
}

/// Disables all interrupts
///
/// Forms a read/write memory barrier for the compiler.
#[inline]
#[asm_cfg(cortex_m)]
pub fn disable() {
    unsafe { asm!("cpsid i", options(nostack, preserves_flags)) };

    // Ensure no subsequent memory accesses are reordered to before interrupts are disabled.
    compiler_fence(Ordering::SeqCst);
}

/// Enables all the interrupts
///
/// Forms a read/write memory barrier for the compiler.
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
#[asm_cfg(cortex_m)]
pub unsafe fn enable() {
    // Ensure no preceeding memory accesses are reordered to after interrupts are enabled.
    compiler_fence(Ordering::SeqCst);

    unsafe { asm!("cpsie i", options(nostack, preserves_flags)) };
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    // Backup previous state of PRIMASK register. We access the entire register directly as a
    // u32 instead of using the primask::read() function to minimize the number of processor
    // cycles during which interrupts are disabled.
    let primask = crate::register::primask::read_raw();

    // disable interrupts
    disable();

    let r = f(&unsafe { CriticalSection::new() });

    unsafe {
        crate::register::primask::write_raw(primask);
    }

    r
}

/// Local definitions of the items this module used to re-export from the
/// deprecated `bare-metal` 0.2 crate. They match `bare-metal` 0.2.5 exactly.
#[cfg(not(feature = "bare-metal"))]
mod compat {
    use core::cell::UnsafeCell;

    /// Critical section token
    ///
    /// Indicates that you are executing code within a critical section
    pub struct CriticalSection {
        _0: (),
    }

    impl CriticalSection {
        /// Creates a critical section token
        ///
        /// This method is meant to be used to create safe abstractions rather than
        /// meant to be directly used in applications.
        ///
        /// # Safety
        ///
        /// The token must only exist while interrupts are disabled (or while
        /// some other mechanism ensures exclusive access on a single core).
        #[inline]
        pub unsafe fn new() -> Self {
            CriticalSection { _0: () }
        }
    }

    /// A "mutex" based on critical sections
    ///
    /// # Safety
    ///
    /// **This Mutex is only safe on single-core systems.**
    ///
    /// On multi-core systems, a `CriticalSection` **is not sufficient** to ensure exclusive access.
    pub struct Mutex<T> {
        inner: UnsafeCell<T>,
    }

    impl<T> Mutex<T> {
        /// Creates a new mutex
        #[inline]
        pub const fn new(value: T) -> Self {
            Mutex {
                inner: UnsafeCell::new(value),
            }
        }

        /// Borrows the data for the duration of the critical section
        #[inline]
        pub fn borrow<'cs>(&'cs self, _cs: &'cs CriticalSection) -> &'cs T {
            unsafe { &*self.inner.get() }
        }
    }

    // NOTE A `Mutex` can be used as a channel so the protected data must be `Send`
    // to prevent sending non-Sendable stuff (e.g. access tokens) across different
    // execution contexts (e.g. interrupts)
    unsafe impl<T> Sync for Mutex<T> where T: Send {}

    /// Interrupt number
    ///
    /// # Safety
    ///
    /// See [`InterruptNumber`](super::InterruptNumber), which supersedes this trait.
    pub unsafe trait Nr {
        /// Returns the number associated with an interrupt
        fn nr(&self) -> u8;
    }

    /// ``` compile_fail
    /// fn bad(cs: &cortex_m::interrupt::CriticalSection) -> &u32 {
    ///     let x = cortex_m::interrupt::Mutex::new(42u32);
    ///     x.borrow(cs)
    /// }
    /// ```
    #[allow(dead_code)]
    const GH_6: () = ();
}
