//! Example demonstrating the skip-data-init feature
//!
//! This feature is useful when using bootloaders (like RP2040's boot2) that:
//! 1. Copy all data from Flash to RAM
//! 2. Unmap the Flash from memory space
//! 3. Jump to the Reset handler
//!
//! In such scenarios, the default cortex-m-rt data initialization would fail
//! because it tries to copy from Flash which is no longer accessible.
//!
//! To use this feature, enable it in your Cargo.toml:
//! ```toml
//! [dependencies]
//! cortex-m-rt = { version = "0.7", features = ["skip-data-init"] }
//! ```
//!
//! And ensure your bootloader or linker script properly initializes .data before
//! jumping to Reset.

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;

static mut COUNTER: u32 = 42;

#[entry]
fn main() -> ! {
    let (prev, new) = unsafe {
        COUNTER += 1;
        (COUNTER - 1, COUNTER)
    };

    hprintln!("Previous counter value: {}", prev);
    hprintln!("New counter value: {}", new);

    if prev != 42 || new != 43 {
        panic!("Unexpected COUNTER value! Data section may not be initialized.");
    }

    loop {
        cortex_m::asm::nop();
    }
}
