#[cfg(all(cortex_m, feature = "single-core-critical-section"))]
mod single_core_critical_section {
    use critical_section::{set_impl, Impl, RawToken};

    use crate::interrupt;
    use crate::register::primask::{self, Primask};

    struct SingleCoreCriticalSection;
    set_impl!(SingleCoreCriticalSection);

    unsafe impl Impl for SingleCoreCriticalSection {
        unsafe fn acquire() -> RawToken {
            match primask::read() {
                Primask::Active => {
                    interrupt::disable();
                    true
                }
                Primask::Inactive => false,
            }
        }

        unsafe fn release(primask_was_active: RawToken) {
            // Only re-enable interrupts if they were enabled before the critical section.
            if primask_was_active {
                interrupt::enable()
            }
        }
    }
}

pub use critical_section::with;
