#![no_main]
#![no_std]

use core::fmt::Write;

#[cortex_m_rt::entry]
fn main() -> ! {
    let x = 42;

    loop {
        let mut hstdout = cortex_m_semihosting::hio::hstdout().unwrap();
        write!(hstdout, "x = {}\n", x).unwrap();
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_SUCCESS);
    }
}

// Define a panic handler that uses semihosting to exit immediately,
// so that any panics cause qemu to quit instead of hang.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_FAILURE);
    }
}
