#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_semihosting;

use cortex_m_rt::{entry, exception};

#[entry]
fn foo() -> ! {
    loop {}
}

#[exception] //~ ERROR custom attribute panicked
//~^ HELP `DefaultHandler` exception must have signature `[unsafe] fn(i16) [-> !]`
fn DefaultHandler(_irqn: i16) -> u32 {
    0
}
