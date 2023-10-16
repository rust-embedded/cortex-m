//! Checks that the declared unsafety is respected by the attributes

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::peripheral::scb::Vector;
use cortex_m_rt::{entry, exception, ExceptionFrame};

#[entry]
unsafe fn main() -> ! {
    foo();

    loop {}
}

#[exception]
unsafe fn DefaultHandler(_: Vector) {
    foo();
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    foo();

    loop {}
}

#[exception]
unsafe fn SysTick() {
    foo();
}

unsafe fn foo() {}
