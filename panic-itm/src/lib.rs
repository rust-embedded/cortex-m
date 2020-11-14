//! Log panic messages using the ITM (Instrumentation Trace Macrocell)
//!
//! This crate contains an implementation of `panic_fmt` that logs panic messages to the ITM
//! stimulus port 0. Before printing the message the panic handler disables (masks) all the device
//! specific interrupts. After printing the message the panic handler goes into an infinite loop.
//!
//! # Usage
//!
//! ``` ignore
//! #![no_std]
//!
//! extern crate panic_itm;
//!
//! fn main() {
//!     panic!("FOO")
//! }
//! ```
//!
//! ``` text
//! (gdb) monitor tpiu config external uart off 8000000 2000000
//! (gdb) monitor itm port 0 on
//! (gdb) continue
//! (..)
//! ```
//!
//! ``` text
//! $ itmdump -f /dev/ttyUSB0
//! panicked at 'FOO', src/main.rs:6:5
//! ```

#![cfg(all(target_arch = "arm", target_os = "none"))]
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate cortex_m;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

use cortex_m::interrupt;
use cortex_m::iprintln;
use cortex_m::peripheral::ITM;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();

    let itm = unsafe { &mut *ITM::ptr() };
    let stim = &mut itm.stim[0];

    iprintln!(stim, "{}", info);

    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
