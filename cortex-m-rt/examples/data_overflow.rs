//! This is not an example; this is a linker overflow detection test
//! which should fail to link due to .data overflowing FLASH.

#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_abort;

use core::ptr;

use rt::ExceptionFrame;

entry!(main);

// This large static array uses most of .rodata
static RODATA: [u8; 48*1024] = [1u8; 48*1024];

// This large mutable array causes .data to use the rest of FLASH
// without also overflowing RAM.
static mut DATA: [u8; 16*1024] = [1u8; 16*1024];

fn main() -> ! {
    unsafe {
        let _bigdata = ptr::read_volatile(&RODATA as *const u8);
        let _bigdata = ptr::read_volatile(&DATA as *const u8);
    }

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

exception!(*, default_handler);

fn default_handler(_irqn: i16) {}
