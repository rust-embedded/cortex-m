use core::sync::atomic::{compiler_fence, Ordering};
use critical_section::{set_impl, Impl, RawRestoreState};

use crate::interrupt;
use crate::register::primask;

struct SingleCoreCriticalSection;
set_impl!(SingleCoreCriticalSection);

unsafe impl Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let restore_state = primask::read();
        // NOTE: Fence guarantees are provided by interrupt::disable(), which performs a `compiler_fence(SeqCst)`.
        interrupt::disable();
        restore_state.0
    }

    unsafe fn release(restore_state: RawRestoreState) {
        // Ensure no preceeding memory accesses are reordered to after interrupts are enabled.
        compiler_fence(Ordering::SeqCst);
        primask::write(restore_state);
    }
}
