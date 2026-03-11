//! Checks that the declared unsafety is respected by the attributes

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{ExceptionFrame, entry, exception};

#[entry]
unsafe fn main() -> ! {
    unsafe { foo() };

    loop {}
}

#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    unsafe { foo() };
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    unsafe { foo() };

    loop {}
}

#[exception]
unsafe fn SysTick() {
    unsafe { foo() };
}

unsafe fn foo() {}
