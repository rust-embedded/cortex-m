//! Manually create the interrupts portion of the vector table

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}

// interrupts portion of the vector table
#[repr(C)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

unsafe extern "C" {
    fn WWDG();
    fn PVD();
}

#[allow(unsafe_code)]
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 3] = [
    Vector { handler: WWDG },
    Vector { reserved: 0 },
    Vector { handler: PVD },
];
