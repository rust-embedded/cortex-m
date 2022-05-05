#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m::peripheral::scb::Vector;
use cortex_m_rt::{entry, exception};

#[entry]
fn foo() -> ! {
    loop {}
}

#[exception]
unsafe fn DefaultHandler(_: Vector, _: u32) {}
//~^ ERROR `DefaultHandler` must have signature `unsafe fn(Vector) [-> !]`
