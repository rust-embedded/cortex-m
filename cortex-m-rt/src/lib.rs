//! Minimal startup / runtime for Cortex-M microcontrollers
//!
//! # Features
//!
//! This crate provides
//!
//! - Before main initialization of the `.bss` and `.data` sections.
//!
//! - Before main initialization of the FPU (for targets that have a FPU).
//!
//! - A `panic_fmt` implementation that just calls abort that you can opt into
//!   through the "abort-on-panic" Cargo feature. If you don't use this feature
//!   you'll have to provide the `panic_fmt` lang item yourself. Documentation
//!   [here][1]
//!
//! [1]: https://doc.rust-lang.org/unstable-book/language-features/lang-items.html
//!
//! - A minimal `start` lang item to support the standard `fn main()`
//!   interface. (The processor goes to sleep (`loop { asm!("wfi") }`) after
//!   returning from `main`)
//!
//! - A linker script that encodes the memory layout of a generic Cortex-M
//!   microcontroller. This linker script is missing some information that must
//!   be supplied through a `memory.x` file (see example below).
//!
//! - A default exception handler tailored for debugging that lets you inspect
//!   what was the state of the processor at the time of the exception. By
//!   default, all exceptions are serviced by this handler but each exception
//!   can be individually overridden using the
//!   [`exception!`](macro.exception.html) macro. The default exception handler
//!   itself can also be overridden using the
//!   [`default_handler!`](macro.default_handler.html) macro.
//!
//! - A `_sheap` symbol at whose address you can locate a heap.
//!
//! # Example
//!
//! Creating a new bare metal project. (I recommend you use the
//! [`cortex-m-quickstart`][qs] template as it takes of all the boilerplate
//! shown here)
//!
//! [qs]: https://docs.rs/cortex-m-quickstart/0.2.0/cortex_m_quickstart/
//!
//! ``` text
//! $ cargo new --bin app && cd $_
//!
//! $ # add this crate as a dependency
//! $ edit Cargo.toml && cat $_
//! [dependencies.cortex-m-rt]
//! features = ["abort-on-panic"]
//! version = "0.3.0"
//!
//! $ # tell Xargo which standard crates to build
//! $ edit Xargo.toml && cat $_
//! [dependencies.core]
//! stage = 0
//!
//! [dependencies.compiler_builtins]
//! features = ["mem"]
//! stage = 1
//!
//! $ # memory layout of the device
//! $ edit memory.x && cat $_
//! MEMORY
//! {
//!   /* NOTE K = KiBi = 1024 bytes */
//!   FLASH : ORIGIN = 0x08000000, LENGTH = 128K
//!   RAM : ORIGIN = 0x20000000, LENGTH = 8K
//! }
//!
//! $ edit src/main.rs && cat $_
//! ```
//!
//! ``` ignore,no_run
//! #![feature(used)]
//! #![no_std]
//!
//! extern crate cortex_m_rt;
//!
//! fn main() {
//!     // do something here
//! }
//!
//! // As we are not using interrupts, we just register a dummy catch all
//! // handler
//! #[link_section = ".vector_table.interrupts"]
//! #[used]
//! static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];
//!
//! extern "C" fn default_handler() {
//!     loop {}
//! }
//! ```
//!
//! ``` text
//! $ cargo install xargo
//!
//! $ xargo rustc --target thumbv7m-none-eabi -- \
//!       -C link-arg=-Tlink.x -C linker=arm-none-eabi-ld -Z linker-flavor=ld
//!
//! $ arm-none-eabi-objdump -Cd $(find target -name app) | head
//!
//! Disassembly of section .text:
//!
//! 08000400 <cortex_m_rt::reset_handler>:
//!  8000400:       b580            push    {r7, lr}
//!  8000402:       466f            mov     r7, sp
//!  8000404:       b084            sub     sp, #8
//! ```
//!
//! # Symbol interfaces
//!
//! This crate makes heavy use of symbols, linker sections and linker scripts to
//! provide most of its functionality. Below are described the main symbol
//! interfaces.
//!
//! ## `DEFAULT_HANDLER`
//!
//! This weak symbol can be overridden to override the default exception handler
//! that this crate provides. It's recommended that you use the
//! `default_handler!` to do the override, but below is shown how to manually
//! override the symbol:
//!
//! ``` ignore,no_run
//! #[no_mangle]
//! pub extern "C" fn DEFAULT_HANDLER() {
//!     // do something here
//! }
//! ```
//!
//! ## `.vector_table.interrupts`
//!
//! This linker section is used to register interrupt handlers in the vector
//! table. The recommended way to use this section is to populate it, once, with
//! an array of *weak* functions that just call the `DEFAULT_HANDLER` symbol.
//! Then the user can override them by name.
//!
//! ### Example
//!
//! Populating the vector table
//!
//! ``` ignore,no_run
//! // Number of interrupts the device has
//! const N: usize = 60;
//!
//! // Default interrupt handler that just calls the `DEFAULT_HANDLER`
//! #[linkage = "weak"]
//! #[naked]
//! #[no_mangle]
//! extern "C" fn WWDG() {
//!     unsafe {
//!         asm!("b DEFAULT_HANDLER" :::: "volatile");
//!         core::intrinsics::unreachable();
//!     }
//! }
//!
//! // You need one function per interrupt handler
//! #[linkage = "weak"]
//! #[naked]
//! #[no_mangle]
//! extern "C" fn WWDG() {
//!     unsafe {
//!         asm!("b DEFAULT_HANDLER" :::: "volatile");
//!         core::intrinsics::unreachable();
//!     }
//! }
//!
//! // ..
//!
//! // Use `None` for reserved spots in the vector table
//! #[link_section = ".vector_table.interrupts"]
//! #[no_mangle]
//! #[used]
//! static INTERRUPTS: [Option<extern "C" fn()>; N] = [
//!     Some(WWDG),
//!     Some(PVD),
//!     // ..
//! ];
//! ```
//!
//! Overriding an interrupt (this can be in a different crate)
//!
//! ``` ignore,no_run
//! // the name must match the name of one of the weak functions used to
//! // populate the vector table.
//! #[no_mangle]
//! pub extern "C" fn WWDG() {
//!     // do something here
//! }
//! ```
//!
//! ## `memory.x`
//!
//! This file supplies the information about the device to the linker.
//!
//! ### `MEMORY`
//!
//! The main information that this file must provide is the memory layout of
//! the device in the form of the `MEMORY` command. The command is documented
//! [here][2], but at a minimum you'll want to create two memory regions: one
//! for Flash memory and another for RAM.
//!
//! [2]: https://sourceware.org/binutils/docs/ld/MEMORY.html
//!
//! The program instructions (the `.text` section) will be stored in the memory
//! region named FLASH, and the program `static` variables (the sections `.bss`
//! and `.data`) will be allocated in the memory region named RAM.
//!
//! ### `_stack_start`
//!
//! This symbol provides the address at which the call stack will be allocated.
//! The call stack grows downwards so this address is usually set to the highest
//! valid RAM address plus one (this *is* an invalid address but the processor
//! will decrement the stack pointer *before* using its value as an address).
//!
//! If omitted this symbol value will default to `ORIGIN(RAM) + LENGTH(RAM)`.
//!
//! #### Example
//!
//! Allocating the call stack on a different RAM region.
//!
//! ```,ignore
//! MEMORY
//! {
//!   /* call stack will go here */
//!   CCRAM : ORIGIN = 0x10000000, LENGTH = 8K
//!   FLASH : ORIGIN = 0x08000000, LENGTH = 256K
//!   /* static variables will go here */
//!   RAM : ORIGIN = 0x20000000, LENGTH = 40K
//! }
//!
//! _stack_start = ORIGIN(CCRAM) + LENGTH(CCRAM);
//! ```
//!
//! ### `_stext`
//!
//! This symbol indicates where the `.text` section will be located. If not
//! specified in the `memory.x` file it will default to right after the vector
//! table -- the vector table is always located at the start of the FLASH
//! region.
//!
//! The main use of this symbol is leaving some space between the vector table
//! and the `.text` section unused. This is required on some microcontrollers
//! that store some configuration information right after the vector table.
//!
//! #### Example
//!
//! Locate the `.text` section 1024 bytes after the start of the FLASH region.
//!
//! ```,ignore
//! _stext = ORIGIN(FLASH) + 0x400;
//! ```
//!
//! ### `_sheap`
//!
//! This symbol is located in RAM right after the `.bss` and `.data` sections.
//! You can use the address of this symbol as the start address of a heap
//! region. This symbol is 4 byte aligned so that address will be a multiple of
//! 4.
//!
//! #### Example
//!
//! ```,ignore
//! extern crate some_allocator;
//!
//! // Size of the heap in bytes
//! const SIZE: usize = 1024;
//!
//! extern "C" {
//!     static mut _sheap: u8;
//! }
//!
//! fn main() {
//!     unsafe {
//!         let start_address = &mut _sheap as *mut u8;
//!         some_allocator::initialize(start_address, SIZE);
//!     }
//! }
//! ```

#![cfg_attr(any(target_arch = "arm", feature = "abort-on-panic"),
            feature(core_intrinsics))]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(naked_functions)]
#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate compiler_builtins;
extern crate r0;

#[cfg(not(test))]
mod lang_items;

#[cfg(target_arch = "arm")]
use core::intrinsics;

#[cfg(target_arch = "arm")]
use cortex_m::asm;
#[cfg(target_arch = "arm")]
use cortex_m::exception::ExceptionFrame;

extern "C" {
    // NOTE `rustc` forces this signature on us. See `src/lang_items.rs`
    #[cfg(target_arch = "arm")]
    fn main(argc: isize, argv: *const *const u8) -> isize;

    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section (stored in Flash)
    static _sidata: u32;
}

#[cfg(target_arch = "arm")]
#[link_section = ".vector_table.reset_vector"]
#[used]
static RESET_VECTOR: unsafe extern "C" fn() -> ! = reset_handler;

/// The reset handler
///
/// This is the entry point of all programs
#[cfg(target_arch = "arm")]
#[link_section = ".reset_handler"]
unsafe extern "C" fn reset_handler() -> ! {
    r0::zero_bss(&mut _sbss, &mut _ebss);
    r0::init_data(&mut _sdata, &mut _edata, &_sidata);

    match () {
        #[cfg(not(has_fpu))]
        () => {
            // Neither `argc` or `argv` make sense in bare metal context so we
            // just stub them
            main(0, ::core::ptr::null());
        }
        #[cfg(has_fpu)]
        () => {
            // NOTE(safe) no exception / interrupt that also accesses the FPU
            // can occur here
            let scb = &*cortex_m::peripheral::SCB.get();
            scb.enable_fpu();

            // Make sure the user main function never gets inlined into this
            // function as that may cause FPU related instructions like vpush to
            // be executed *before* enabling the FPU and that would generate an
            // exception
            #[inline(never)]
            fn main() {
                unsafe {
                    ::main(0, ::core::ptr::null());
                }
            }

            main()
        }
    }

    // If `main` returns, then we go into "reactive" mode and simply attend
    // interrupts as they occur.
    loop {
        asm!("wfi" :::: "volatile");
    }
}

#[cfg(target_arch = "arm")]
global_asm!(r#"
.weak NMI
NMI = DEFAULT_HANDLER

.weak HARD_FAULT
HARD_FAULT = DEFAULT_HANDLER

.weak MEM_MANAGE
MEM_MANAGE = DEFAULT_HANDLER

.weak BUS_FAULT
BUS_FAULT = DEFAULT_HANDLER

.weak USAGE_FAULT
USAGE_FAULT = DEFAULT_HANDLER

.weak SVCALL
SVCALL = DEFAULT_HANDLER

.weak PENDSV
PENDSV = DEFAULT_HANDLER

.weak SYS_TICK
SYS_TICK = DEFAULT_HANDLER
"#);

#[cfg(not(armv6m))]
global_asm!(r#"
.weak DEBUG_MONITOR
DEBUG_MONITOR = DEFAULT_HANDLER
"#);

#[cfg(target_arch = "arm")]
extern "C" {
    fn NMI();
    fn HARD_FAULT();
    fn MEM_MANAGE();
    fn BUS_FAULT();
    fn USAGE_FAULT();
    fn SVCALL();
    #[cfg(not(armv6m))]
    fn DEBUG_MONITOR();
    fn PENDSV();
    fn SYS_TICK();
}

#[allow(private_no_mangle_statics)]
#[cfg(target_arch = "arm")]
#[doc(hidden)]
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
#[used]
pub static EXCEPTIONS: [Option<unsafe extern "C" fn()>; 14] = [
    Some(NMI),
    Some(HARD_FAULT),
    Some(MEM_MANAGE),
    Some(BUS_FAULT),
    Some(USAGE_FAULT),
    None,
    None,
    None,
    None,
    Some(SVCALL),
    #[cfg(armv6m)]
    None,
    #[cfg(not(armv6m))]
    Some(DEBUG_MONITOR),
    None,
    Some(PENDSV),
    Some(SYS_TICK),
];

/// `ef` points to the exception frame
///
/// That exception frame is a snapshot of the program state right before the
/// exception occurred.
#[allow(unused_variables)]
#[cfg(target_arch = "arm")]
extern "C" fn default_handler(ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {}

    #[export_name = "DEFAULT_HANDLER"]
    #[linkage = "weak"]
    #[naked]
    extern "C" fn trampoline() -> ! {
        unsafe {
            asm!("mrs r0, MSP
                 b $0"
                 :
                 : "i"(default_handler as extern "C" fn(&ExceptionFrame) -> !)
                 :
                 : "volatile");

            intrinsics::unreachable()
        }
    }

    #[used]
    static KEEP: extern "C" fn() -> ! = trampoline;
}

// make sure the compiler emits the DEFAULT_HANDLER symbol so the linker can
// find it!
#[cfg(target_arch = "arm")]
#[used]
static KEEP: extern "C" fn(&ExceptionFrame) -> ! = default_handler;

/// This macro lets you override the default exception handler
///
/// The first and only argument to this macro is the path to the function that
/// will be used as the default handler. That function must have signature
/// `fn()`
///
/// # Examples
///
/// ``` ignore
/// default_handler!(foo::bar);
///
/// mod foo {
///     pub fn bar() {
///         ::cortex_m::asm::bkpt();
///         loop {}
///     }
/// }
/// ```
#[macro_export]
macro_rules! default_handler {
    ($path:path) => {
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern "C" fn DEFAULT_HANDLER() {
            // type checking
            let f: fn() = $path;
            f();
        }
    }
}

/// Fault and system exceptions
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub enum Exception {
    /// Non-maskable interrupt
    NMI,
    /// All class of fault.
    HARD_FAULT,
    /// Memory management.
    MEN_MANAGE,
    /// Pre-fetch fault, memory access fault.
    BUS_FAULT,
    /// Undefined instruction or illegal state.
    USAGE_FAULT,
    /// System service call via SWI instruction
    SVCALL,
    /// Debug monitor
    #[cfg(not(armv6m))]
    DEBUG_MONITOR,
    /// Pendable request for system service
    PENDSV,
    /// System tick timer
    SYS_TICK,
}

/// Assigns a handler to an exception
///
/// This macro takes two arguments: the name of an exception and the path to the
/// function that will be used as the handler of that exception. That function
/// must have signature `fn()`.
///
/// Optionally, a third argument may be used to declare exception local data.
/// The handler will have exclusive access to these *local* variables on each
/// invocation. If the third argument is used then the signature of the handler
/// function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first
/// argument passed to the macro.
///
/// # Example
///
/// ``` ignore
/// exception!(MEM_MANAGE, mpu_fault);
///
/// fn mpu_fault() {
///     panic!("Oh no! Something went wrong");
/// }
///
/// exception!(SYS_TICK, periodic, locals: {
///     counter: u32 = 0;
/// });
///
/// fn periodic(locals: &mut SYS_TICK::Locals) {
///     locals.counter += 1;
///     println!("This function has been called {} times", locals.counter);
/// }
/// ```
#[macro_export]
macro_rules! exception {
    ($NAME:ident, $path:path, locals: {
        $($lvar:ident:$lty:ident = $lval:expr;)+
    }) => {
        #[allow(non_snake_case)]
        mod $NAME {
            pub struct Locals {
                $(
                    pub $lvar: $lty,
                )+
            }
        }

        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern "C" fn $NAME() {
            // check that the handler exists
            let _ = $crate::Exception::$NAME;

            static mut LOCALS: self::$NAME::Locals = self::$NAME::Locals {
                $(
                    $lvar: $lval,
                )*
            };

            // type checking
            let f: fn(&mut self::$NAME::Locals) = $path;
            f(&mut LOCALS);
        }
    };
    ($NAME:ident, $path:path) => {
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern "C" fn $NAME() {
            // check that the handler exists
            let _ = $crate::Exception::$NAME;

            // type checking
            let f: fn() = $path;
            f();
        }
    }
}
