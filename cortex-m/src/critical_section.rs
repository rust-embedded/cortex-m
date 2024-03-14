use critical_section::{set_impl, Impl, RawRestoreState};

use crate::interrupt;
use crate::register::primask;

struct SingleCoreCriticalSection;
set_impl!(SingleCoreCriticalSection);

unsafe impl Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let was_active = primask::read().is_active();
        // NOTE: Fence guarantees are provided by interrupt::disable(), which performs a `compiler_fence(SeqCst)`.
        interrupt::disable();
        was_active
    }

    unsafe fn release(was_active: RawRestoreState) {
        // Only re-enable interrupts if they were enabled before the critical section.
        if was_active {
            // NOTE: Fence guarantees are provided by interrupt::enable(), which performs a
            // `compiler_fence(SeqCst)`.
            interrupt::enable()
        }
    }
}
