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

#[interrupt]
fn USART1() -> i32 {
    //~^ ERROR `#[interrupt]` handlers must have signature `[unsafe] fn() [-> !]`
    0
}
