#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception, interrupt};

#[no_mangle]
#[entry]
fn foo() -> ! {
    loop {}
}

#[no_mangle]
#[exception]
fn SysTick() {}

#[allow(non_camel_case_types)]
enum interrupt {
    USART1,
    USART2,
}

#[no_mangle]
#[interrupt]
fn USART1() {}

#[cfg(feature = "device")]
#[cfg_attr(feature = "device", no_mangle)]
#[interrupt]
fn USART2() {}
