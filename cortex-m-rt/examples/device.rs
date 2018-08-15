//! Manually create the interrupts portion of the vector table

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

// the program entry point
entry!(main);

fn main() -> ! {
    loop {}
}

// interrupts portion of the vector table
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

extern "C" {
    fn WWDG();
    fn PVD();
}

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 3] = [
    Vector { handler: WWDG },
    Vector { reserved: 0 },
    Vector { handler: PVD },
];
