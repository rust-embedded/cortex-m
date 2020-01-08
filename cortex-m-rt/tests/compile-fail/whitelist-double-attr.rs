#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception};

#[exception]
#[entry] //~ ERROR this attribute is not allowed on a function controlled by cortex-m-rt
fn SVCall() -> ! {
    loop {}
}
