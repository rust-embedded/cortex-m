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

#[interrupt] //~ ERROR custom attribute panicked
//~^ HELP `#[interrupt]` functions must have signature `[unsafe] fn() [-> !]`
fn USART1(undef: i32) {}
