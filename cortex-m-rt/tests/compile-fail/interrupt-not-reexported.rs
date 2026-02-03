#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, interrupt};

#[entry]
fn foo() -> ! {
    loop {}
}

#[interrupt] //~ ERROR failed to resolve: use of unresolved module or unlinked crate `interrupt`
fn USART1() {}
