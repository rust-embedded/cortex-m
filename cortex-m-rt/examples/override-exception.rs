//! How to override the hard fault exception handler and the default exception handler

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::{asm, peripheral::scb::Vector};
use cortex_m_rt::{entry, exception, ExceptionFrame};

#[entry]
fn main() -> ! {
    loop {}
}

#[exception]
unsafe fn DefaultHandler(_: Vector) {
    asm::bkpt();
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {}
}
