#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception, interrupt};

#[inline]
#[entry]
fn foo() -> ! {
    loop {}
}

#[inline]
#[exception]
fn SysTick() {}

#[allow(non_camel_case_types)]
enum interrupt {
    USART1,
    USART2,
}

#[inline]
#[interrupt]
fn USART1() {}

#[cfg(feature = "device")]
#[cfg_attr(feature = "device", inline)]
#[interrupt]
fn USART2() {}
