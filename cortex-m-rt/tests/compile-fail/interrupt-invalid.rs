#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, interrupt};

#[entry]
fn foo() -> ! {
    loop {}
}

enum interrupt {
    USART1,
}

// NOTE this looks a bit better when using a device crate:
// "no variant named `foo` found for type `stm32f30x::Interrupt` in the current scope"
#[interrupt] //~ ERROR no variant named `foo` found for type `interrupt` in the current scope
fn foo() {}
