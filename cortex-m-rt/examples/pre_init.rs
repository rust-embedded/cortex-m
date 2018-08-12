//! `cortex-m-rt` based program with a function run before RAM is initialized.

#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, pre_init)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

pre_init!(disable_watchdog);

unsafe fn disable_watchdog() {
    // Do what you need to disable the watchdog.
}

// the program entry point
entry!(main);

fn main() -> ! {
    loop {}
}
