//! Minimal startup / runtime for Cortex-M microcontrollers
//!
//! This crate contains all the required parts to build a `no_std` application (binary crate) that
//! targets a Cortex-M microcontroller.
//!
//! # Features
//!
//! This crates takes care of:
//!
//! - The memory layout of the program. In particular, it populates the vector table so the device
//! can boot correctly, and properly dispatch exceptions and interrupts.
//!
//! - Initializing `static` variables before the program entry point.
//!
//! - Enabling the FPU before the program entry point if the target is `thumbv7em-none-eabihf`.
//!
//! This crate also provides a mechanism to set exception handlers: see the [`exception!`] macro.
//!
//! [`exception!`]: macro.exception.html
//!
//! # Requirements
//!
//! ## `arm-none-eabi-gcc`
//!
//! This crate requires `arm-none-eabi-gcc` to be installed and available in `$PATH`.
//!
//! ## `memory.x`
//!
//! This crate expects the user, or some other crate, to provide the memory layout of the target
//! device via a linker script named `memory.x`. This section covers the contents of `memory.x`
//!
//! ### `MEMORY`
//!
//! The linker script must specify the memory available in the device as, at least, two `MEMORY`
//! regions: one named `FLASH` and one named `RAM`. The `.text` and `.rodata` sections of the
//! program will be placed in the `FLASH` region, whereas the `.bss` and `.data` sections, as well
//! as the heap,will be placed in the `RAM` region.
//!
//! ``` text
//! /* Linker script for the STM32F103C8T6 */
//! MEMORY
//! {
//!   FLASH : ORIGIN = 0x08000000, LENGTH = 64K
//!   RAM : ORIGIN = 0x20000000, LENGTH = 20K
//! }
//! ```
//!
//! ### `_stack_start`
//!
//! This optional symbol can be used to indicate where the call stack of the program should be
//! placed. If this symbol is not used then the stack will be placed at the *end* of the `RAM`
//! region -- the stack grows downwards towards smaller address. This symbol can be used to place
//! the stack in a different memory region, for example:
//!
//! ``` text
//! /* Linker script for the STM32F303VCT6 */
//! MEMORY
//! {
//!     FLASH : ORIGIN = 0x08000000, LENGTH = 256K
//!
//!     /* .bss, .data and the heap go in this region */
//!     RAM : ORIGIN = 0x20000000, LENGTH = 40K
//!
//!     /* Core coupled (faster) RAM dedicated to hold the stack */
//!     CCRAM : ORIGIN = 0x10000000, LENGTH = 8K
//! }
//!
//! _stack_start = ORIGIN(CCRAM) + LENGTH(CCRAM);
//! ```
//!
//! ### `_stext`
//!
//! This optional symbol can be used to control where the `.text` section is placed. If omitted the
//! `.text` section will be placed right after the vector table, which is placed at the beginning of
//! `FLASH`. Some devices store settings like Flash configuration right after the vector table;
//! for these devices one must place the `.text` section after this configuration section --
//! `_stext` can be used for this purpose.
//!
//! ``` text
//! MEMORY
//! {
//!   /* .. */
//! }
//!
//! /* The device stores Flash configuration in 0x400-0x40C so we place .text after that */
//! _stext = ORIGIN(FLASH) + 0x40C
//! ```
//!
//! # An example
//!
//! This section presents a minimal application built on top of `cortex-m-rt`. Apart from the
//! mandatory `memory.x` linker script describing the memory layout of the device, the hard fault
//! handler and the default exception handler must also be defined somewhere in the dependency
//! graph (cf. [`exception!`]). In this example we define them in the binary crate:
//!
//! ``` ignore
//! // IMPORTANT the standard `main` interface is not used because it requires nightly
//! #![no_main]
//! #![no_std]
//!
//! #[macro_use(entry, exception)]
//! extern crate cortex_m_rt as rt;
//!
//! // makes `panic!` print messages to the host stderr using semihosting
//! extern crate panic_semihosting;
//!
//! use rt::ExceptionFrame;
//!
//! // use `main` as the entry point of this application
//! entry!(main);
//!
//! // `main` is not allowed to return
//! fn main() -> ! {
//!     // initialization
//!
//!     loop {
//!         // application logic
//!     }
//! }
//!
//! // define the hard fault handler
//! exception!(HardFault, hard_fault);
//!
//! fn hard_fault(ef: &ExceptionFrame) -> ! {
//!     panic!("{:#?}", ef);
//! }
//!
//! // define the default exception handler
//! exception!(*, default_handler);
//!
//! fn default_handler(irqn: i16) {
//!     panic!("unhandled exception (IRQn={})", irqn);
//! }
//! ```
//!
//! To actually build this program you need to place a `memory.x` linker script somewhere the linker
//! can find it, e.g. in the current directory; and then link the program using `cortex-m-rt`'s
//! linker script: `link.x`. The required steps are shown below:
//!
//! ``` text
//! $ cat > memory.x <<EOF
//! /* Linker script for the STM32F103C8T6 */
//! MEMORY
//! {
//!   FLASH : ORIGIN = 0x08000000, LENGTH = 64K
//!   RAM : ORIGIN = 0x20000000, LENGTH = 20K
//! }
//! EOF
//!
//! $ cargo rustc --target thumbv7m-none-eabi -- \
//!       -C link-arg=-nostartfiles -C link-arg=-Tlink.x
//!
//! $ file target/thumbv7m-none-eabi/debug/app
//! app: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, (..)
//! ```
//!
//! # Optional features
//!
//! ## `device`
//!
//! If this feature is disabled then this crate populates the whole vector table. All the interrupts
//! in the vector table, even the ones unused by the target device, will be bound to the default
//! exception handler. This makes the final application device agnostic: you will be able to run it
//! on any Cortex-M device -- provided that you correctly specified its memory layout in `memory.x`
//! -- without hitting undefined behavior.
//!
//! If this feature is enabled then the interrupts section of the vector table is left unpopulated
//! and some other crate, or the user, will have to populate it. This mode is meant to be used in
//! conjunction with crates generated using `svd2rust`. Those *device crates* will populate the
//! missing part of the vector table when their `"rt"` feature is enabled.
//!
//! # Inspection
//!
//! This section covers how to inspect a binary that builds on top of `cortex-m-rt`.
//!
//! ## Sections (`size`)
//!
//! `cortex-m-rt` uses standard sections like `.text`, `.rodata`, `.bss` and `.data` as one would
//! expect. `cortex-m-rt` separates the vector table in its own section, named `.vector_table`. This
//! lets you distinguish how much space is taking the vector table in Flash vs how much is being
//! used by actual instructions (`.text`) and constants (`.rodata`).
//!
//! ```
//! $ size -Ax target/thumbv7m-none-eabi/examples/app
//! target/thumbv7m-none-eabi/release/examples/app  :
//! section             size         addr
//! .vector_table      0x400    0x8000000
//! .text               0x88    0x8000400
//! .rodata              0x0    0x8000488
//! .data                0x0   0x20000000
//! .bss                 0x0   0x20000000
//! ```
//!
//! Without the `-A` argument `size` reports the sum of the sizes of `.text`, `.rodata` and
//! `.vector_table` under "text".
//!
//! ```
//! $ size target/thumbv7m-none-eabi/examples/app
//!   text    data     bss     dec     hex filename
//!   1160       0       0    1660     67c target/thumbv7m-none-eabi/release/app
//! ```
//!
//! ## Symbols (`objdump`, `nm`)
//!
//! One will always find the following (unmangled) symbols in `cortex-m-rt` applications:
//!
//! - `Reset`. This is the reset handler. The microcontroller will executed this function upon
//! booting. This function will call the user program entry point (cf. [`entry!`]) using the `main`
//! symbol so you may also find that symbol in your program; if you do, `main` will contain your
//! application code. Some other times `main` gets inlined into `Reset` so you won't find it.
//!
//! [`entry!`]:  macro.entry.html
//!
//! - `DefaultHandler`. This is the default handler. This function will contain, or call, the
//! function you declared in the second argument of `exception!(*, ..)`.
//!
//! - `HardFault`. This is the hard fault handler. This function is simply a trampoline that jumps
//! into the user defined hard fault handler: `UserHardFault`. The trampoline is required to set up
//! the pointer to the stacked exception frame.
//!
//! - `UserHardFault`. This is the user defined hard fault handler. This function will contain, or
//! call, the function you declared in the second argument of `exception!(HardFault, ..)`
//!
//! - `__STACK_START`. This is the first entry in the `.vector_table` section. This symbol contains
//! the initial value of the stack pointer; this is where the stack will be located -- the stack
//! grows downwards towards smaller addresses.
//!
//! - `__RESET_VECTOR`. This is the reset vector, a pointer into the `Reset` handler. This vector is
//! located in the `.vector_table` section after `__STACK_START`.
//!
//! - `__EXCEPTIONS`. This is the core exceptions portion of the vector table; it's an array of 14
//! exception vectors, which includes exceptions like `HardFault` and `SysTick`. This array is
//! located after `__RESET_VECTOR` in the `.vector_table` section.
//!
//! - `__EXCEPTIONS`. This is the device specific interrupt portion of the vector table; its exact
//! size depends on the target device but if the `"device"` feature has not been enabled it will
//! have a size of 32 vectors (on ARMv6-M) or 240 vectors (on ARMv7-M). This array is located after
//! `__EXCEPTIONS` in the `.vector_table` section.
//!
//! If you override any exception handler you'll find it as an unmangled symbol, e.g. `SysTick` or
//! `SVCall`, in the output of `objdump`,
//!
//! If you are targeting the `thumbv7em-none-eabihf` target you'll also see a `ResetTrampoline`
//! symbol in the output. To avoid the compiler placing FPU instructions before the FPU has been
//! enabled (cf. `vpush`) `Reset` calls the function `ResetTrampoline` which is marked as
//! `#[inline(never)]` and `ResetTrampoline` calls `main`. The compiler is free to inline `main`
//! into `ResetTrampoline` but it can't inline `ResetTrampoline` into `Reset` -- the FPU is enabled
//! in `Reset`.
//!
//! # Advanced usage
//!
//! ## Setting the program entry point
//!
//! This section describes how `entry!` is implemented. This information is useful to developers who
//! want to provide an alternative to `entry!` that provides extra guarantees.
//!
//! The `Reset` handler will call a symbol named `main` (unmangled) *after* initializing `.bss` and
//! `.data`, and enabling the FPU (if the target is `thumbv7em-none-eabihf`). `entry!` provides this
//! symbol in its expansion:
//!
//! ``` ignore
//! entry!(path::to::main);
//!
//! // expands into
//!
//! #[export_name = "main"]
//! pub extern "C" fn __impl_main() -> ! {
//!     // validate the signature of the program entry point
//!     let f: fn() -> ! = path::to::main;
//!
//!     f()
//! }
//! ```
//!
//! The unmangled `main` symbol must have signature `extern "C" fn() -> !` or its invocation from
//! `Reset`  will result in undefined behavior.
//!
//! ## Incorporating device specific interrupts
//!
//! This section covers how an external crate can insert device specific interrupt handlers into the
//! vector table. Most users don't need to concern themselves with these details, but if you are
//! interested in how device crates generated using `svd2rust` integrate with `cortex-m-rt` read on.
//!
//! The information in this section applies when the `"device"` feature has been enabled.
//!
//! ### `__INTERRUPTS`
//!
//! The external crate must provide the interrupts portion of the vector table via a `static`
//! variable named`__INTERRUPTS` (unmangled) that must be placed in the `.vector_table.interrupts`
//! section of its object file.
//!
//! This `static` variable will be placed at `ORIGIN(FLASH) + 0x40`. This address corresponds to the
//! spot where IRQ0 (IRQ number 0) is located.
//!
//! To conform to the Cortex-M ABI `__INTERRUPTS` must be an array of function pointers; some spots
//! in this array may need to be set to 0 if they are marked as *reserved* in the data sheet /
//! reference manual. We recommend using a `union` to set the reserved spots to `0`; `None`
//! (`Option<fn()>`) may also work but it's not guaranteed that the `None` variant will *always* be
//! represented by the value `0`.
//!
//! Let's illustrate with an artificial example where a device only has two interrupt: `Foo`, with
//! IRQ number = 2, and `Bar`, with IRQ number = 4.
//!
//! ``` ignore
//! union Vector {
//!     handler: extern "C" fn(),
//!     reserved: usize,
//! }
//!
//! extern "C" {
//!     fn Foo();
//!     fn Bar();
//! }
//!
//! #[link_section = ".vector_table.interrupts"]
//! #[no_mangle]
//! pub static __INTERRUPTS: [Vector; 5] = [
//!     // 0-1: Reserved
//!     Vector { reserved: 0 },
//!     Vector { reserved: 0 },
//!
//!     // 2: Foo
//!     Vector { handler: Foo },
//!
//!     // 3: Reserved
//!     Vector { reserved: 0 },
//!
//!     // 4: Bar
//!     Vector { handler: Bar },
//! ];
//! ```
//!
//! ### `device.x`
//!
//! Linking in `__INTERRUPTS` creates a bunch of undefined references. If the user doesn't set a
//! handler for *all* the device specific interrupts then linking will fail with `"undefined
//! reference"` errors.
//!
//! We want to provide a default handler for all the interrupts while still letting the user
//! individually override each interrupt handler. In C projects, this is usually accomplished using
//! weak aliases declared in external assembly files. In Rust, we could achieve something similar
//! using `global_asm!`, but that's an unstable feature.
//!
//! A solution that doesn't require `global_asm!` or external assembly files is to use the `PROVIDE`
//! command in a linker script to create the weak aliases. This is the approach that `cortex-m-rt`
//! uses; when the `"device"` feature is enabled `cortex-m-rt`'s linker script (`link.x`) depends on
//! a linker script named `device.x`. The crate that provides `__INTERRUPTS` must also provide this
//! file.
//!
//! For our running example the `device.x` linker script looks like this:
//!
//! ``` text
//! /* device.x */
//! PROVIDE(Foo = DefaultHandler);
//! PROVIDE(Bar = DefaultHandler);
//! ```
//!
//! This weakly aliases both `Foo` and `Bar`. `DefaultHandler` is the default exception handler that
//! the user provides via `exception!(*, ..)` and that the core exceptions use unless overridden.
//!
//! Because this linker script is provided by a dependency of the final application the dependency
//! must contain build script that puts `device.x` somewhere the linker can find. An example of such
//! build script is shown below:
//!
//! ``` ignore
//! use std::env;
//! use std::fs::File;
//! use std::io::Write;
//! use std::path::PathBuf;
//!
//! fn main() {
//!     // Put the linker script somewhere the linker can find it
//!     let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
//!     File::create(out.join("device.x"))
//!         .unwrap()
//!         .write_all(include_bytes!("device.x"))
//!         .unwrap();
//!     println!("cargo:rustc-link-search={}", out.display());
//! }
//! ```

// # Developer notes
//
// - `link_section` is used to place symbols in specific places of the final binary. The names used
// here will appear in the linker script (`link.x`) in conjunction with the `KEEP` command.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate r0;

use core::{fmt, ptr};

/// Registers stacked (pushed into the stack) during an exception
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExceptionFrame {
    /// (General purpose) Register 0
    pub r0: u32,

    /// (General purpose) Register 1
    pub r1: u32,

    /// (General purpose) Register 2
    pub r2: u32,

    /// (General purpose) Register 3
    pub r3: u32,

    /// (General purpose) Register 12
    pub r12: u32,

    /// Linker Register
    pub lr: u32,

    /// Program Counter
    pub pc: u32,

    /// Program Status Register
    pub xpsr: u32,
}

impl fmt::Debug for ExceptionFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(u32);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0x{:08x}", self.0)
            }
        }
        f.debug_struct("ExceptionFrame")
            .field("r0", &Hex(self.r0))
            .field("r1", &Hex(self.r1))
            .field("r2", &Hex(self.r2))
            .field("r3", &Hex(self.r3))
            .field("r12", &Hex(self.r12))
            .field("lr", &Hex(self.lr))
            .field("pc", &Hex(self.pc))
            .field("xpsr", &Hex(self.xpsr))
            .finish()
    }
}

/// Returns a pointer to the start of the heap
///
/// The returned pointer is guaranteed to be 4-byte aligned.
#[inline]
pub fn heap_start() -> *mut u32 {
    extern "C" {
        static mut __sheap: u32;
    }

    unsafe { &mut __sheap }
}

/* Entry point */
#[doc(hidden)]
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        // This symbol will be provided by the user via the `entry!` macro
        fn main() -> !;

        // These symbols come from `link.x`
        static mut __sbss: u32;
        static mut __ebss: u32;

        static mut __sdata: u32;
        static mut __edata: u32;
        static __sidata: u32;
    }

    // Initialize RAM
    r0::zero_bss(&mut __sbss, &mut __ebss);
    r0::init_data(&mut __sdata, &mut __edata, &__sidata);

    match () {
        #[cfg(not(has_fpu))]
        () => main(),
        #[cfg(has_fpu)]
        () => {
            // We redefine these here to avoid pulling the `cortex-m` crate as a dependency
            const SCB_CPACR: *mut u32 = 0xE000_ED88 as *mut u32;
            const SCB_CPACR_FPU_ENABLE: u32 = 0b01_01 << 20;
            const SCB_CPACR_FPU_USER: u32 = 0b10_10 << 20;

            // enable the FPU
            core::ptr::write_volatile(
                SCB_CPACR,
                *SCB_CPACR | SCB_CPACR_FPU_ENABLE | SCB_CPACR_FPU_USER,
            );

            // this is used to prevent the compiler from inlining the user `main` into the reset
            // handler. Inlining can cause the FPU instructions in the user `main` to be executed
            // before enabling the FPU, and that would produce a hard to diagnose hard fault at
            // runtime.
            #[inline(never)]
            #[export_name = "ResetTrampoline"]
            fn trampoline() -> ! {
                unsafe { main() }
            }

            trampoline()
        }
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultDefaultHandler() {
    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728
        ptr::read_volatile(&0u8);
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultUserHardFault() {
    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728
        ptr::read_volatile(&0u8);
    }
}

/// Macro to define the entry point of the program
///
/// **NOTE** This macro must be invoked once and must be invoked from an accessible module, ideally
/// from the root of the crate.
///
/// Usage: `entry!(path::to::entry::point)`
///
/// The specified function will be called by the reset handler *after* RAM has been initialized. In
/// the case of the `thumbv7em-none-eabihf` target the FPU will also be enabled before the function
/// is called.
///
/// The signature of the specified function must be `fn() -> !` (never ending function)
#[macro_export]
macro_rules! entry {
    ($path:expr) => {
        #[export_name = "main"]
        pub extern "C" fn __impl_main() -> ! {
            // validate the signature of the program entry point
            let f: fn() -> ! = $path;

            f()
        }
    };
}

/* Exceptions */
#[doc(hidden)]
pub enum Exception {
    NonMaskableInt,

    // Not overridable
    // HardFault,
    #[cfg(not(armv6m))]
    MemoryManagement,

    #[cfg(not(armv6m))]
    BusFault,

    #[cfg(not(armv6m))]
    UsageFault,

    #[cfg(armv8m)]
    SecureFault,

    SVCall,

    #[cfg(not(armv6m))]
    DebugMonitor,

    PendSV,

    SysTick,
}

extern "C" {
    fn NonMaskableInt();

    fn HardFault();

    #[cfg(not(armv6m))]
    fn MemoryManagement();

    #[cfg(not(armv6m))]
    fn BusFault();

    #[cfg(not(armv6m))]
    fn UsageFault();

    #[cfg(armv8m)]
    fn SecureFault();

    fn SVCall();

    #[cfg(not(armv6m))]
    fn DebugMonitor();

    fn PendSV();

    fn SysTick();
}

#[doc(hidden)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[doc(hidden)]
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Vector; 14] = [
    // Exception 2: Non Maskable Interrupt.
    Vector {
        handler: NonMaskableInt,
    },
    // Exception 3: Hard Fault Interrupt.
    Vector { handler: HardFault },
    // Exception 4: Memory Management Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector {
        handler: MemoryManagement,
    },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // Exception 5: Bus Fault Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector { handler: BusFault },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // Exception 6: Usage Fault Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector {
        handler: UsageFault,
    },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // Exception 7: Secure Fault Interrupt [only on Armv8-M].
    #[cfg(armv8m)]
    Vector {
        handler: SecureFault,
    },
    #[cfg(not(armv8m))]
    Vector { reserved: 0 },
    // 8-10: Reserved
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    // Exception 11: SV Call Interrupt.
    Vector { handler: SVCall },
    // Exception 12: Debug Monitor Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector {
        handler: DebugMonitor,
    },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // 13: Reserved
    Vector { reserved: 0 },
    // Exception 14: Pend SV Interrupt [not on Cortex-M0 variants].
    Vector { handler: PendSV },
    // Exception 15: System Tick Interrupt.
    Vector { handler: SysTick },
];

// If we are not targeting a specific device we bind all the potential device specific interrupts
// to the default handler
#[cfg(all(not(feature = "device"), not(armv6m)))]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 240] = [{
    extern "C" {
        fn DefaultHandler();
    }

    DefaultHandler
}; 240];

// ARMv6-M can only have a maximum of 32 device specific interrupts
#[cfg(all(not(feature = "device"), armv6m))]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 32] = [{
    extern "C" {
        fn DefaultHandler();
    }

    DefaultHandler
}; 32];

/// Macro to set or override a processor core exception handler
///
/// **NOTE** This macro must be invoked from an accessible module, ideally from the root of the
/// crate.
///
/// # Syntax
///
/// ``` ignore
/// exception!(
///     // Name of the exception
///     $Name:ident,
///
///     // Path to the exception handler (a function)
///     $handler:expr,
///
///     // Optional, state preserved across invocations of the handler
///     state: $State:ty = $initial_state:expr,
/// );
/// ```
///
/// where `$Name` can be one of:
///
/// - `*`
/// - `NonMaskableInt`
/// - `HardFault`
/// - `MemoryManagement` (a)
/// - `BusFault` (a)
/// - `UsageFault` (a)
/// - `SecureFault` (b)
/// - `SVCall`
/// - `DebugMonitor` (a)
/// - `PendSV`
/// - `SysTick`
///
/// (a) Not available on Cortex-M0 variants (`thumbv6m-none-eabi`)
///
/// (b) Only available on ARMv8-M
///
/// # Usage
///
/// `exception!(HardFault, ..)` sets the hard fault handler. The handler must have signature
/// `fn(&ExceptionFrame) -> !`. This handler is not allowed to return as that can cause undefined
/// behavior. It's mandatory to set the `HardFault` handler somewhere in the dependency graph of an
/// application.
///
/// `exception!(*, ..)` sets the *default* handler. All exceptions which have not been assigned a
/// handler will be serviced by this handler. This handler must have signature `fn(irqn: i16)`.
/// `irqn` is the IRQ number (cf. CMSIS); `irqn` will be a negative number when the handler is
/// servicing a core exception; `irqn` will be a positive number when the handler is servicing a
/// device specific exception (interrupt). It's mandatory to set the default handler somewhere
/// in the dependency graph of an application.
///
/// `exception!($Exception, ..)` overrides the default handler for `$Exception`. All exceptions,
/// except for `HardFault`, can be assigned some `$State`.
///
/// # Examples
///
/// - Setting the `HardFault` handler
///
/// ```
/// #[macro_use(exception)]
/// extern crate cortex_m_rt as rt;
///
/// use rt::ExceptionFrame;
///
/// exception!(HardFault, hard_fault);
///
/// fn hard_fault(ef: &ExceptionFrame) -> ! {
///     // prints the exception frame as a panic message
///     panic!("{:#?}", ef);
/// }
///
/// # fn main() {}
/// ```
///
/// - Setting the default handler
///
/// ```
/// #[macro_use(exception)]
/// extern crate cortex_m_rt as rt;
///
/// exception!(*, default_handler);
///
/// fn default_handler(irqn: i16) {
///     println!("IRQn = {}", irqn);
/// }
///
/// # fn main() {}
/// ```
///
/// - Overriding the `SysTick` handler
///
/// ```
/// #[macro_use(exception)]
/// extern crate cortex_m_rt as rt;
///
/// exception!(SysTick, sys_tick, state: u32 = 0);
///
/// fn sys_tick(count: &mut u32) {
///     println!("count = {}", *count);
///
///     *count += 1;
/// }
///
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! exception {
    (* , $handler:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)] // raise an error if this item is not accessible
        #[no_mangle]
        pub unsafe extern "C" fn DefaultHandler() {
            extern crate core;

            // validate the signature of the user provided handler
            let f: fn(i16) = $handler;

            const SCB_ICSR: *const u32 = 0xE000_ED04 as *const u32;

            // NOTE not volatile so the compiler can opt the load operation away if the value is
            // unused
            f(core::ptr::read(SCB_ICSR) as u8 as i16 - 16)
        }
    };

    (HardFault, $handler:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)] // raise an error if this item is not accessible
        #[no_mangle]
        pub unsafe extern "C" fn UserHardFault(ef: &$crate::ExceptionFrame) {
            // validate the signature of the user provided handler
            let f: fn(&$crate::ExceptionFrame) -> ! = $handler;

            f(ef)
        }
    };

    ($Name:ident, $handler:expr,state: $State:ty = $initial_state:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)] // raise an error if this item is not accessible
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;

            // check that this exception exists
            let _ = $crate::Exception::$Name;

            // validate the signature of the user provided handler
            let f: fn(&mut $State) = $handler;

            f(&mut STATE)
        }
    };

    ($Name:ident, $handler:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)] // raise an error if this item is not accessible
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            // check that this exception exists
            let _ = $crate::Exception::$Name;

            // validate the signature of the user provided handler
            let f: fn() = $handler;

            f()
        }
    };
}
