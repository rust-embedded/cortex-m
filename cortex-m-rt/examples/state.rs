//! Preserving state across executions of an exception handler

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

// the program entry point
entry!(main);

fn main() -> ! {
    loop {}
}

// exception handler with state
exception!(SysTick, sys_tick, state: u32 = 0);

fn sys_tick(state: &mut u32) {
    *state += 1;
}
