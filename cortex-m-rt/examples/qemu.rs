// #![feature(stdsimd)]
#![no_main]
#![no_std]

extern crate cortex_m;

extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as semihosting;
extern crate panic_semihosting;
extern crate unreachable;

use core::fmt::Write;
use cortex_m::asm;
use rt::{entry, exception, ExceptionFrame};

#[entry]
fn main() -> ! {
    let x = 42;

    loop {
        asm::nop();

        // write something through semihosting interface
        let mut hstdout = semihosting::hio::hstdout().unwrap();
        write!(hstdout, "x = {}\n", x);

        asm::nop();

        // exit from qemu
        semihosting::debug::exit(semihosting::debug::EXIT_SUCCESS);

        // hint to the optimizer that any code path which calls this function is statically unreachable
        unsafe {
            unreachable::unreachable();
        }
    }
}

#[exception]
#[inline(always)]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    loop {
        asm::nop()
    }
}

#[exception]
#[inline(always)]
fn DefaultHandler(_irqn: i16) {}

// use core::panic::PanicInfo;
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {{
//     semihosting::debug::exit(semihosting::debug::EXIT_SUCCESS)}
// }
