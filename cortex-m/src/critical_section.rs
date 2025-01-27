#[cfg(all(cortex_m, feature = "critical-section-single-core"))]
mod single_core_critical_section {
    use critical_section::{set_impl, Impl, RawRestoreState};

    use crate::interrupt;
    use crate::register::primask;

    struct SingleCoreCriticalSection;
    set_impl!(SingleCoreCriticalSection);

    unsafe impl Impl for SingleCoreCriticalSection {
        unsafe fn acquire() -> RawRestoreState {
            // Backup previous state of PRIMASK register. We access the entire register directly as a
            // u32 instead of using the primask::read() function to minimize the number of processor
            // cycles during which interrupts are disabled.
            let restore_state = primask::read_raw();
            // NOTE: Fence guarantees are provided by interrupt::disable(), which performs a `compiler_fence(SeqCst)`.
            interrupt::disable();
            restore_state
        }

        unsafe fn release(restore_state: RawRestoreState) {
            // NOTE: Fence guarantees are provided by primask::write_raw(), which performs a `compiler_fence(SeqCst)`.
            primask::write_raw(restore_state);
        }
    }
}
