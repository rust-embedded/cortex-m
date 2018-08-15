//! How to override the hard fault exception handler and the default exception handler

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use cortex_m::asm;
use rt::ExceptionFrame;

// the program entry point
entry!(main);

fn main() -> ! {
    loop {}
}

exception!(*, default_handler);

fn default_handler(_irqn: i16) {
    asm::bkpt();
}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {}
}
