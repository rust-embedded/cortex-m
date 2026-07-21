//! How to override the hard fault exception handler and the default exception handler

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use rt::{ExceptionFrame, entry, exception};

#[entry]
fn main() -> ! {
    loop {}
}

#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    unsafe {
        core::arch::asm!("bkpt");
    }
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    unsafe {
        core::arch::asm!("bkpt");
    }

    loop {}
}
