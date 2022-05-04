use crate::interrupt;
use crate::register::primask::{self, Primask};

struct CriticalSection;
critical_section::custom_impl!(CriticalSection);

const TOKEN_IGNORE: u8 = 0;
const TOKEN_REENABLE: u8 = 1;

unsafe impl critical_section::Impl for CriticalSection {
    unsafe fn acquire() -> u8 {
        match primask::read() {
            Primask::Active => {
                interrupt::disable();
                TOKEN_REENABLE
            }
            Primask::Inactive => TOKEN_IGNORE,
        }
    }

    unsafe fn release(token: u8) {
        // Only re-enable interrupts if they were enabled before the critical section.
        if token == TOKEN_REENABLE {
            interrupt::enable()
        }
    }
}
