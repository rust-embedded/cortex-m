//! Preserving state across executions of an exception handler

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use rt::ExceptionFrame;

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

// the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

// the default exception handler
exception!(*, default_handler);

fn default_handler(_irqn: i16) {}
