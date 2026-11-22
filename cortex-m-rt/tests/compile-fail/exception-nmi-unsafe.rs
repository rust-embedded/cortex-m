#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception};

#[entry]
fn foo() -> ! {
    loop {}
}

#[exception]
fn DefaultHandler(_irq: i16) {}

#[exception]
fn HardFault() {}

#[exception]
fn NonMaskableInt() {}
