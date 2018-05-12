//! Directly plug a `main` symbol instead of using `entry!`

#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use rt::ExceptionFrame;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
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
