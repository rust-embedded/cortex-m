#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_semihosting;

use cortex_m_rt::{entry, exception};

#[entry]
fn foo() -> ! {
    loop {}
}

#[exception]
fn DefaultHandler(_irqn: i16) {}

pub mod reachable {
    use cortex_m_rt::exception;

    #[exception] //~ ERROR symbol `DefaultHandler` is already defined
    fn DefaultHandler(_irqn: i16) {}
}
