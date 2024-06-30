//! This is not an example; this is a linker overflow detection test
//! which should fail to link due to .data overflowing FLASH.

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use core::ptr;

use rt::entry;

// Large static arrays uses most of .rodata
static RODATA_0: [u8; 8 * 1024] = [1u8; 8 * 1024];
static RODATA_1: [u8; 8 * 1024] = [1u8; 8 * 1024];
static RODATA_2: [u8; 8 * 1024] = [1u8; 8 * 1024];
static RODATA_3: [u8; 8 * 1024] = [1u8; 8 * 1024];
static RODATA_4: [u8; 8 * 1024] = [1u8; 8 * 1024];
static RODATA_5: [u8; 8 * 1024] = [1u8; 8 * 1024];

// These large mutable arrays causes .data to use the rest of FLASH
// without also overflowing RAM.
static mut DATA_0: [u8; 8 * 1024] = [1u8; 8 * 1024];
static mut DATA_1: [u8; 8 * 1024] = [1u8; 8 * 1024];

#[entry]
fn main() -> ! {
    unsafe {
        let _ = ptr::read_volatile(ptr::addr_of!(RODATA_0));
        let _ = ptr::read_volatile(ptr::addr_of!(RODATA_1));
        let _ = ptr::read_volatile(ptr::addr_of!(RODATA_2));
        let _ = ptr::read_volatile(ptr::addr_of!(RODATA_3));
        let _ = ptr::read_volatile(ptr::addr_of!(RODATA_4));
        let _ = ptr::read_volatile(ptr::addr_of!(RODATA_5));

        let _ = ptr::read_volatile(ptr::addr_of!(DATA_0));
        let _ = ptr::read_volatile(ptr::addr_of!(DATA_1));
    }

    loop {}
}
