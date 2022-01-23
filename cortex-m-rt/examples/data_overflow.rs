//! This is not an example; this is a linker overflow detection test
//! which should fail to link due to .data overflowing FLASH.

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use core::ptr;

use rt::entry;

// This large static array uses most of .rodata
static RODATA: [u8; 48 * 1024] = [1u8; 48 * 1024];

// This large mutable array causes .data to use the rest of FLASH
// without also overflowing RAM.
static mut DATA: [u8; 16 * 1024] = [1u8; 16 * 1024];

#[entry]
fn main() -> ! {
    unsafe {
        let _bigdata = ptr::read_volatile(&RODATA as *const u8);
        let _bigdata = ptr::read_volatile(&DATA as *const u8);
    }

    loop {}
}
