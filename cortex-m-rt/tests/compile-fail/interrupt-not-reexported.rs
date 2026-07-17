#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, interrupt};

#[entry]
fn foo() -> ! {
    loop {}
}

#[interrupt] //~ ERROR cannot find module or crate `interrupt`
fn USART1() {}
