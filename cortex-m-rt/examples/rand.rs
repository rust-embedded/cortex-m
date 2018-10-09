//! Use rand crate to ensure it's configured for no_std compatbility

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
use rt::entry;

extern crate panic_halt;

extern crate rand;
use rand::Rng;
use rand::SeedableRng;

// the program entry point
#[entry]
fn main() -> ! {
    let seed: [u8; 32] = [0; 32];
    let mut rng = rand::ChaChaRng::from_seed(seed);
    let _ = rng.gen::<u32>();

    loop {}
}
