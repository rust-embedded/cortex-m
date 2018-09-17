#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::entry;

#[entry(foo)] //~ ERROR custom attribute panicked
//~^ HELP `entry` attribute must have no arguments
fn foo() -> ! {
    loop {}
}
