//! Interrupts

pub use bare_metal::{CriticalSection, Mutex, Nr};

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

/// Implement InterruptNumber for the old bare_metal::Nr trait.
/// This implementation is for backwards compatibility only and will be removed in cortex-m 0.8.
unsafe impl<T: Nr + Copy> InterruptNumber for T {
    #[inline]
    fn number(self) -> u16 {
        self.nr() as u16
    }
}

/// Disables all interrupts
#[inline]
pub fn disable() {
    call_asm!(__cpsid());
}

/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn enable() {
    call_asm!(__cpsie());
}

/// Hacky compatibility layer to allow calling `interrupt::free` using
/// closures with arity 0 as well as 1. This trait is not considered
/// part of the public API.
///
/// The generic `Args` type is not actually used, see:
/// https://geo-ant.github.io/blog/2021/rust-traits-and-variadic-functions/
///
/// TODO: Remove before releasing 0.8.
pub trait InterruptFreeFn<Args, R> {
    /// Call the closure.
    unsafe fn call(self) -> R;
}

impl<F, R> InterruptFreeFn<(), R> for F
where
    F: FnOnce() -> R,
{
    #[inline]
    unsafe fn call(self) -> R {
        self()
    }
}

impl<'cs, F, R> InterruptFreeFn<&'cs CriticalSection, R> for F
where
    F: FnOnce(&'cs CriticalSection) -> R,
{
    #[inline]
    unsafe fn call(self) -> R {
        let cs: &'cs CriticalSection = core::mem::transmute(&CriticalSection::new());
        self(cs)
    }
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[inline]
pub fn free<Args, F, R>(f: F) -> R
where
    F: InterruptFreeFn<Args, R>,
{
    let primask = crate::register::primask::read();

    // disable interrupts
    disable();

    let r = unsafe { f.call() };

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    if primask.is_active() {
        unsafe { enable() }
    }

    r
}
