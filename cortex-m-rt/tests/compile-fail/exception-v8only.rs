#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception};

#[entry]
fn foo() -> ! {
    loop {}
}

#[exception]
fn SecureFault() {}
//~^ ERROR no variant, associated function, or constant named `SecureFault` found for enum `cortex_m_rt::Exception` in the current scope [E0599]
