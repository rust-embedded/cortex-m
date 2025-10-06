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

use panic_halt as _;

use cortex_m_rt::entry;

static mut COUNTER: u32 = 42;

#[entry]
fn main() -> ! {
    unsafe {
        COUNTER += 1;
    }
    
    loop {
        cortex_m::asm::nop();
    }
}
