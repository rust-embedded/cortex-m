//! Minimal startup / runtime for Cortex-M microcontrollers
//!
//! # Features
//!
//! This crate provides
//!
//! - Before main initialization of the `.bss` and `.data` sections
//!
//! - An overridable (\*) `panic_fmt` implementation that prints to the ITM or
//!   to the host stdout (through semihosting) depending on which Cargo feature
//!   has been enabled: `"panic-over-itm"` or `"panic-over-semihosting"`.
//!
//! - A minimal `start` lang item, to support vanilla `fn main()`. NOTE the
//!   processor goes into "reactive" mode (`loop { asm!("wfi") }`) after
//!   returning from `main`.
//!
//! - An opt-in linker script (`"linker-script"` Cargo feature) that encodes
//!   the memory layout of a generic Cortex-M microcontroller. This linker
//!   script is missing the definition of the FLASH and RAM memory regions of
//!   the device. This missing information must be supplied through a `memory.x`
//!   linker script of the form:
//!
//! ``` text
//! MEMORY
//! {
//!   FLASH : ORIGIN = 0x08000000, LENGTH = 128K
//!   RAM : ORIGIN = 0x20000000, LENGTH = 8K
//! }
//! ```
//!
//! - A default exception handler tailored for debugging and that provides
//!   access to the stacked registers under the debugger. By default, all
//!   exceptions (\*\*) are serviced by this handler but this can be overridden
//!   on a per exception basis by opting out of the "exceptions" Cargo feature
//!   and then defining the following `struct`
//!
//! ``` ignore,no_run
//! use cortex_m::exception;
//!
//! #[link_section = ".rodata.exceptions"]
//! #[used]
//! static EXCEPTIONS: exception::Handlers = exception::Handlers {
//!     hard_fault: my_override,
//!     nmi: another_handler,
//!     ..exception::DEFAULT_HANDLERS
//! };
//! ````
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
//!
//! (\*\*) All the device specific exceptions, i.e. the interrupts, are left
//! unpopulated. You must fill that part of the vector table by defining the
//! following static (with the right memory layout):
//!
//! ``` ignore,no_run
//! #[link_section = ".rodata.interrupts"]
//! #[used]
//! static INTERRUPTS: SomeStruct = SomeStruct { .. }
//! ```
//!
//! # Usage
//!
//! ``` text
//! $ cargo new --bin app && cd $_
//!
//! $ cargo add cortex-m cortex-m-rt
//!
//! $ cat Xargo.toml
//! ```
//!
//! ``` text
//! [dependencies.core]
//!
//! [dependencies.compiler_builtins]
//! features = ["mem"]
//! git = "https://github.com/rust-lang-nursery/compiler-builtins"
//! stage = 1
//! ```
//!
//! ``` text
//! $ cat memory.x
//! ```
//!
//! ``` text
//! MEMORY
//! {
//!   FLASH : ORIGIN = 0x08000000, LENGTH = 128K
//!   RAM : ORIGIN = 0x20000000, LENGTH = 8K
//! }
//! ```
//!
//! ``` text
//! $ cat src/main.rs
//! ```
//!
//! ``` ignore,no_run
//! #![feature(used)]
//! #![no_std]
//!
//! #[macro_use]
//! extern crate cortex_m;
//! extern crate cortex_m_rt;
//!
//! fn main() {
//!     hprintln!("Hello, world!");
//! }
//!
//! #[allow(dead_code)]
//! #[link_section = ".rodata.interrupts"]
//! #[used]
//! static INTERRUPTS: [u32; 240] = [0; 240];
//! ```
//!
//! ``` text
//! $ xargo rustc --target thumbv7m-none-eabi -- -C link-args='-Tlink.x -nostartfiles'
//!
//! $ arm-none-eabi-objdump -Cd $(find target -name app) | less
//! 08000000 <_VECTOR_TABLE>:
//!  8000000:       20002000        .word   0x20002000
//!
//! 08000004 <cortex_m_rt::RESET_HANDLER>:
//!  8000004:       08000671                                q...
//!
//! 08000008 <cortex_m_rt::EXCEPTIONS>:
//!  8000008:       080005a5 080005bd 08000569 08000599     ........i.......
//!  8000018:       08000581 00000000 00000000 00000000     ................
//!  8000028:       00000000 080005b1 00000000 00000000     ................
//!  8000038:       0800058d 08000575                       ....u...
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
    }

    ::r0::zero_bss(&mut _sbss, &mut _ebss);
    ::r0::init_data(&mut _sdata, &mut _edata, &_sidata);

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
