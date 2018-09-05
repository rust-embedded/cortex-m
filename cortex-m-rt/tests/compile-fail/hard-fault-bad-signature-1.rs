#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_semihosting;

use cortex_m_rt::{entry, exception, ExceptionFrame};

#[entry]
fn foo() -> ! {
    loop {}
}

#[exception] //~ ERROR custom attribute panicked
//~^ HELP `HardFault` exception must have signature `fn(&ExceptionFrame) -> !`
fn HardFault(_ef: &ExceptionFrame, undef: u32) -> ! {
    loop {}
}
