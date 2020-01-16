#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception};

#[exception]
#[entry] //~ ERROR cortex-m-rt does not support multiple attributes on the same function
fn SVCall() -> ! {
    loop {}
}
