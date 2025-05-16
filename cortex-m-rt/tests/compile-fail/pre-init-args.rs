#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, pre_init};

#[pre_init(foo)] //~ ERROR This attribute accepts no arguments
                 //~^ WARNING Use core::arch::global_asm! to define the __pre_init function instead
unsafe fn foo() {}

#[entry]
fn baz() -> ! {
    loop {}
}
