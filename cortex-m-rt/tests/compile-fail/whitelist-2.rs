#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception, interrupt};

#[export_name = "not_allowed"]
#[entry]
fn foo() -> ! {
    loop {}
}

#[export_name = "not_allowed"]
#[exception]
fn SysTick() {}

#[allow(non_camel_case_types)]
enum interrupt {
    USART1,
    USART2,
}

#[export_name = "not_allowed"]
#[interrupt]
fn USART1() {}

#[cfg(feature = "device")]
#[cfg_attr(feature = "device", export_name = "not_allowed")]
#[interrupt]
fn USART2() {}
