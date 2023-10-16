#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::peripheral::scb::Vector;
use cortex_m_rt::{entry, exception};

#[entry]
fn foo() -> ! {
    loop {}
}

#[exception]
unsafe fn DefaultHandler(_: Vector) {}
