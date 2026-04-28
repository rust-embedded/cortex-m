#![no_std]
#![no_main]

use cortex_m_rt::entry;
#[cfg(feature = "hardware")]
use defmt_rtt as _;
#[cfg(feature = "qemu")]
use defmt_semihosting as _;

#[entry]
fn main() -> ! {
    defmt::warn!("=== Use `cargo test` to run the tests ===");
    loop {
        cortex_m::asm::nop();
    }
}

#[cfg(target_env = "")] // appease clippy
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::nop();
    }
}
