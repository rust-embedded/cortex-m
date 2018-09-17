#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception};

#[entry]
fn foo() -> ! {
    loop {}
}

#[exception(SysTick)] //~ ERROR custom attribute panicked
//~^ HELP `exception` attribute must have no arguments
fn SysTick() {}
