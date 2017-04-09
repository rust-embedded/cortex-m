//! Minimal startup / runtime for Cortex-M microcontrollers
//!
//! Provides
//!
//! - Before main initialization of the `.bss` and `.data` sections
//!
//! - An overridable (\*) `panic_fmt` implementation that prints overs the ITM
//!   or through semihosting depending on the enabled Cargo feature.
//!
//! - Minimal `start` lang item, to support vanilla `fn main()`. NOTE the
//!   processor goes into "reactive" mode (`loop { asm!("wfi") }`) after
//!   returning from `main`.
//!
//! (\*) To override the `panic_fmt` implementation, simply create a new
//! `rust_begin_unwind` symbol:
//!
//! ```
//! #[no_mangle]
//! pub unsafe extern "C" fn rust_begin_unwind(
//!     _args: ::core::fmt::Arguments,
//!     _file: &'static str,
//!     _line: u32,
//! ) -> ! {
//!     ..
//! }
//! ```

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(used)]
#![no_std]

#[cfg(any(feature = "panic-over-itm", feature = "exceptions"))]
#[cfg_attr(feature = "panic-over-itm", macro_use)]
extern crate cortex_m;
extern crate compiler_builtins;
#[cfg(feature = "panic-over-semihosting")]
#[macro_use]
extern crate cortex_m_semihosting;
extern crate r0;

mod lang_items;

#[cfg(feature = "exceptions")]
use cortex_m::exception;

/// The reset handler
///
/// This is the entry point of all programs
unsafe extern "C" fn reset_handler() -> ! {
    extern "C" {
        static mut _ebss: u32;
        static mut _sbss: u32;

        static mut _edata: u32;
        static mut _sdata: u32;

        static _sidata: u32;

        static _init_array_start: extern "C" fn();
        static _init_array_end: extern "C" fn();
    }

    ::r0::zero_bss(&mut _sbss, &mut _ebss);
    ::r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    ::r0::run_init_array(&_init_array_start, &_init_array_end);

    // NOTE `rustc` forces this signature on us. See `src/lang_items.rs`
    extern "C" {
        fn main(argc: isize, argv: *const *const u8) -> isize;
    }

    // Neither `argc` or `argv` make sense in bare metal contexts so we just
    // stub them
    main(0, ::core::ptr::null());

    // If `main` returns, then we go into "reactive" mode and attend interrupts
    // as they occur.
    loop {
        asm!("wfi" :::: "volatile");
    }
}

#[allow(dead_code)]
#[used]
#[link_section = ".rodata.reset_handler"]
static RESET_HANDLER: unsafe extern "C" fn() -> ! = reset_handler;

#[allow(dead_code)]
#[cfg(feature = "exceptions")]
#[link_section = ".rodata.exceptions"]
#[used]
static EXCEPTIONS: exception::Handlers = exception::Handlers {
    ..exception::DEFAULT_HANDLERS
};
