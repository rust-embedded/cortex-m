//! Report panic messages to the host stderr using semihosting
//!
//! This crate contains an implementation of `panic_fmt` that logs panic messages to the host stderr
//! using [`cortex-m-semihosting`]. Before logging the message the panic handler disables (masks)
//! the device specific interrupts. After logging the message the panic handler trigger a breakpoint
//! and then goes into an infinite loop.
//!
//! Currently, this crate only supports the ARM Cortex-M architecture.
//!
//! [`cortex-m-semihosting`]: https://crates.io/crates/cortex-m-semihosting
//!
//! # Usage
//!
//! ``` ignore
//! #![no_std]
//!
//! extern crate panic_semihosting;
//!
//! fn main() {
//!     panic!("FOO")
//! }
//! ```
//!
//! ``` text
//! (gdb) monitor arm semihosting enable
//! (gdb) continue
//! Program received signal SIGTRAP, Trace/breakpoint trap.
//! rust_begin_unwind (args=..., file=..., line=8, col=5)
//!     at $CRATE/src/lib.rs:69
//! 69          asm::bkpt();
//! ```
//!
//! ``` text
//! $ openocd -f (..)
//! (..)
//! panicked at 'FOO', src/main.rs:6:5
//! ```
//!
//! # Optional features
//!
//! ## `exit`
//!
//! When this feature is enabled the panic handler performs an exit semihosting call after logging
//! the panic message. This is useful when emulating the program on QEMU as it causes the QEMU
//! process to exit with a non-zero exit code; thus it can be used to implement Cortex-M tests that
//! run on the host.
//!
//! We discourage using this feature when the program will run on hardware as the exit call can
//! leave the hardware debugger in an inconsistent state.
//!
//! ## `inline-asm`
//!
//! When this feature is enabled semihosting is implemented using inline assembly (`asm!`) and
//! compiling this crate requires nightly.
//!
//! When this feature is disabled semihosting is implemented using FFI calls into an external
//! assembly file and compiling this crate works on stable and beta.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_semihosting as sh;

use core::fmt::Write;
use core::panic::PanicInfo;

#[cfg(not(feature = "exit"))]
use cortex_m::asm;
use cortex_m::interrupt;
#[cfg(feature = "exit")]
use sh::debug::{self, EXIT_FAILURE};
use sh::hio;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();

    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{}", info).ok();
    }

    match () {
        // Exit the QEMU process
        #[cfg(feature = "exit")]
        () => debug::exit(EXIT_FAILURE),
        // OK to fire a breakpoint here because we know the microcontroller is connected to a
        // debugger
        #[cfg(not(feature = "exit"))]
        () => asm::bkpt(),
    }

    loop {}
}
