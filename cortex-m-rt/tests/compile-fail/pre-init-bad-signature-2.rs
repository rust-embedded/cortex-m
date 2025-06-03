#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, pre_init};

#[pre_init] //~ WARNING Use core::arch::global_asm! to define the __pre_init function instead
unsafe fn foo(undef: i32) {}
//~^ ERROR `#[pre_init]` function must have signature `unsafe fn()`

#[entry]
fn bar() -> ! {
    loop {}
}
