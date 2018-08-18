//! Preserving state across executions of an exception handler

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use rt::{entry, exception};

#[entry]
fn main() -> ! {
    loop {}
}

// exception handler with state
#[exception(SysTick, static STATE: u32 = 0)]
fn sys_tick() {
    *STATE += 1;
}
