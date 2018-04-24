//! Minimal startup / runtime for Cortex-M microcontrollers

// # Developer notes
//
// - `link_section` is used to place symbols in specific places if the final binary. The names used
// here will appear in the linker script (`link.x`) in conjunction with the `KEEP` command.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate r0;

/// Returns a pointer into which the heap can be placed
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
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = __reset;

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn __reset() -> ! {
    extern "C" {
        // This symbol will be provided by the user via the `main!` macro
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
            #[export_name = "__reset_trampoline"]
            fn trampoline() -> ! {
                unsafe { main() }
            }

            trampoline()
        }
    }
}

/// Macro to define the user entry point of a program
///
/// Usage: `main!(path::to::user::main)`
///
/// This function will be called by the reset handler *after* RAM has been initialized. In the case
/// of the `thumbv7em-none-eabihf` target the FPU will also be enabled before the user `main` is
/// called.
#[macro_export]
macro_rules! main {
    ($path:path) => {
        #[export_name = "main"]
        pub extern "C" fn __impl_main() -> ! {
            // validate the signature of the user provide `main`
            let f: fn() -> ! = $path;

            f()
        }
    };
}

/* Exceptions */
// NOTE we purposefully go against Rust style here and use PascalCase for the handlers. The
// rationale is in the definition of the `exception!` macro.
#[doc(hidden)]
pub enum Exception {
    NMI,
    MemManage,
    BusFault,
    UsageFault,
    SVC,
    DebugMon,
    PendSV,
    SysTick,
}

extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVC();
    #[cfg(not(armv6m))]
    fn DebugMon();
    fn PendSV();
    fn SysTick();
}

#[doc(hidden)]
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Option<unsafe extern "C" fn()>; 14] = [
    Some(NMI),
    Some(HardFault),
    Some(MemManage),
    Some(BusFault),
    Some(UsageFault),
    None,
    None,
    None,
    None,
    Some(SVC),
    #[cfg(armv6m)]
    None,
    #[cfg(not(armv6m))]
    Some(DebugMon),
    None,
    Some(PendSV),
    Some(SysTick),
];

/// Macro to override an exception handler
///
/// Usage: `exception!(ExceptionName, path::to::handler)`
///
/// All exceptions are serviced by the `DefaultHandler` exception handler unless overridden using
/// this macro. `ExceptionName` can be one of are:
///
/// - `DefaultHandler` (\*)
/// - `NMI`
/// - `HardFault`
/// - `MemManage`
/// - `BusFault`
/// - `UsageFault`
/// - `SVC`
/// - `DebugMon` (not available on ARMv6-M)
/// - `PendSV`
/// - `SysTick`
///
/// (\*) Note that `DefaultHandler` is left undefined and *must* be defined by the user somewhere
/// using this macro.
#[macro_export]
macro_rules! exception {
    (DefaultHandler, $path:path) => {
        #[allow(non_snake_case)]
        #[export_name = "DefaultHandler"]
        pub unsafe extern "C" fn __impl_DefaultHandler() {
            // XXX should we really prevent this handler from returning?
            // validate the signature of the user provided handler
            let f: fn() -> ! = $path;

            f()
        }
    };
    (HardFault, $path:path) => {
        #[allow(non_snake_case)]
        #[export_name = "HardFaultr"]
        pub unsafe extern "C" fn __impl_HardFault() {
            // XXX should we really prevent this handler from returning?
            // validate the signature of the user provided handler
            let f: fn() -> ! = $path;

            f()
        }
    };
    // NOTE Unfortunately, this will end up leaking `$exception` into the function call namespace.
    // But the damage is somewhat reduced by having `$exception` not being a `snake_case` function.
    ($ExceptionName:ident, $path:path) => {
        #[no_mangle]
        pub unsafe extern "C" fn $ExceptionName() {
            // check that this exception exists
            let _ = $crate::Exception::$ExceptionName;

            // validate the signature of the user provided handler
            let f: fn() = $path;

            f()
        }
    };
}
