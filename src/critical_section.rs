#[cfg(all(cortex_m, feature = "single-core-critical-section"))]
mod single_core_critical_section {
    use critical_section::{set_impl, Impl, RawToken};

    use crate::interrupt;
    use crate::register::primask::{self, Primask};

    struct SingleCoreCriticalSection;
    set_impl!(SingleCoreCriticalSection);

    const TOKEN_IGNORE: RawToken = 0;
    const TOKEN_REENABLE: RawToken = 1;

    unsafe impl Impl for SingleCoreCriticalSection {
        unsafe fn acquire() -> RawToken {
            match primask::read() {
                Primask::Active => {
                    interrupt::disable();
                    TOKEN_REENABLE
                }
                Primask::Inactive => TOKEN_IGNORE,
            }
        }

        unsafe fn release(token: RawToken) {
            // Only re-enable interrupts if they were enabled before the critical section.
            if token == TOKEN_REENABLE {
                interrupt::enable()
            }
        }
    }
}

pub use critical_section::with;
