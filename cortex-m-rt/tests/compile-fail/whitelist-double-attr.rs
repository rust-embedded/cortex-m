#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception};

#[exception]
#[entry] //~ ERROR multiple cortex-m-rt attributes are not supported on the same function
fn SVCall() -> ! {
    loop {}
}
