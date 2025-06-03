//! `cortex-m-rt` based program with a function run before RAM is initialized.

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use rt::entry;

// This function is called before the RAM is initialized.
// For example, it can be used to disable the watchdog.
core::arch::global_asm! {
    r#"
__pre_init:
    // Do what you need to do before RAM is initialized.
    bx lr
    "#
}

#[entry]
fn main() -> ! {
    loop {}
}
