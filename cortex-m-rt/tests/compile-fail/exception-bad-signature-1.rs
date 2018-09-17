#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception};

#[entry]
fn foo() -> ! {
    loop {}
}

#[exception] //~ ERROR custom attribute panicked
//~^ HELP `#[exception]` functions other than `DefaultHandler` and `HardFault` must have signature `[unsafe] fn() [-> !]`
fn SysTick(undef: u32) {}
