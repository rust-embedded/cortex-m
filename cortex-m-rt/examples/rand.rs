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
    let seed: [u8; 16] = [0; 16];
    let mut rng = rand::rngs::SmallRng::from_seed(seed);
    let _ = rng.gen::<u32>();

    loop {}
}
