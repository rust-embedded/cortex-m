// ignore-test :sadface: it's not possible to prevent this user error at compile time
// see rust-lang/rust#53975 for details

#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::entry;

#[entry]
fn foo() -> ! {
    loop {}
}

mod hidden {
    use cortex_m::peripheral::scb::Vector;
    use cortex_m_rt::exception;

    #[exception]
    unsafe fn DefaultHandler(_: Vector) {}
}
