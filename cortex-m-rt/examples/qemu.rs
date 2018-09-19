#![feature(stdsimd)]
#![no_main]
#![no_std]

extern crate cortex_m;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as semihosting;
extern crate panic_abort;
extern crate unreachable;

use core::arch::arm;
use core::fmt::Write;
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let x = 42;

    loop {
        unsafe { arm::__NOP() }

        // write something through semihosting interface
        let mut hstdout = semihosting::hio::hstdout().unwrap();
        write!(hstdout, "x = {}\n", x);

        // exit from qemu
        semihosting::debug::exit(semihosting::debug::EXIT_SUCCESS);

        // hint to the optimizer that any code path which calls this function is statically unreachable
        unsafe {
            unreachable::unreachable();
        }
    }
}

// define the hard fault handler
exception!(HardFault, hard_fault);

#[inline(always)]
fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {
        unsafe { arm::__NOP() }
    }
}

// define the default exception handler
exception!(*, default_handler);

#[inline(always)]
fn default_handler(_irqn: i16) {}
