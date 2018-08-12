//! `cortex-m-rt` based program with a function run before RAM is initialized.

#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception, pre_init)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use rt::ExceptionFrame;

pre_init!(disable_watchdog);

unsafe fn disable_watchdog() {
    // Do what you need to disable the watchdog.
}

// the program entry point
entry!(main);

fn main() -> ! {
    loop {}
}

// the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

// the default exception handler
exception!(*, default_handler);

fn default_handler(_irqn: i16) {}
