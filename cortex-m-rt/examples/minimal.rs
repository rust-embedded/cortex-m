//! Minimal `cortex-m-rt` based program

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

// the program entry point
entry!(main);

fn main() -> ! {
    loop {}
}
