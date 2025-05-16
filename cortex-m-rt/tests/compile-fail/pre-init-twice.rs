#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, pre_init};

#[pre_init] //~ WARNING Use core::arch::global_asm! to define the __pre_init function instead
unsafe fn foo() {}

#[pre_init] //~ ERROR symbol `__pre_init` is already defined
            //~^ WARNING Use core::arch::global_asm! to define the __pre_init function instead
unsafe fn bar() {}

#[entry]
fn baz() -> ! {
    loop {}
}
