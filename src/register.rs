//! Processor core registers
//!
//! The following registers can only be accessed in PRIVILEGED mode:
//!
//! - MSP
//! - IPSR
//! - EPSR
//! - PRIMASK
//! - FAULTMASK
//! - BASEPRI
//! - CONTROL
//!
//! The rest of registers (see list below) can be accessed in either, PRIVILEGED or UNPRIVILEGED,
//! mode.
//!
//! - PSP
//! - LR
//! - PC
//! - APSR
//!
//! # Caveats
//!
//! - The API doesn't check if the value passed to `write` is valid (e.g. reserved bits are not
//!   modified) or not. It's up to the user to verify that.
//!
//! # References
//!
//! - Cortex-M* Devices Generic User Guide - Section 2.1.3 Core registers

// NOTE all the functions here are `always(inline)` to prevent a function call which may change the
// contents of the core registers.

macro_rules! sr {
    ($name:ident) => {
        /// Reads the special register
        #[inline(always)]
        pub unsafe fn read() -> u32 {
            let r: u32;
            match () {
                #[cfg(target_arch = "arm")]
                () => asm!(concat!("mrs ", "$0,", stringify!($name)) : "=r"(r) ::: "volatile"),

                #[cfg(not(target_arch = "arm"))]
                () => r = 0,
            }
            r
        }
    };
}

macro_rules! srw {
    (#[$attr:meta] $name:ident) => {
        #[$attr]
        pub mod $name {
            sr!($name);

            /// Writes to the special register
            #[inline(always)]
            pub unsafe fn write(r: u32) {
                match r {
                    #[cfg(target_arch = "arm")]
                    _ => asm!(concat!("msr ", stringify!($name), ",$0") :: "r"(r) ::: "volatile"),

                    #[cfg(not(target_arch = "arm"))]
                    _ => {},
                }
            }
        }
    };
}

macro_rules! sro {
    (#[$attr:meta] $name:ident) => {
        #[$attr]
        pub mod $name {
            sr!($name);
        }
    }
}

macro_rules! rw {
    (#[$attr:meta] $name:ident : $r:ident) => {
        #[$attr]
        pub mod $name {
            /// Reads the special register
            #[inline(always)]
            pub unsafe fn read() -> u32 {
                let r: u32;
                match () {
                    #[cfg(target_arch = "arm")]
                    () => asm!(concat!("mov ", "$0,", stringify!($r)) : "=r"(r) ::: "volatile"),

                    #[cfg(not(target_arch = "arm"))]
                    () => r = 0,
                }
                r
            }

            /// Writes to the special register
            #[inline(always)]
            pub unsafe fn write(r: u32) {
                match r {
                    #[cfg(target_arch = "arm")]
                    _ => asm!(concat!("mov ", stringify!($r), ",$0") :: "r"(r) ::: "volatile"),

                    #[cfg(not(target_arch = "arm"))]
                    _ => {}
                }
            }
        }
    }
}

srw!(#[doc = "Main Stack Pointer"] msp);
srw!(#[doc = "Process Stack Pointer"] psp);
rw!(#[doc = "Link Register"] lr: r14);
rw!(#[doc = "Program Counter"] pc: r15);
srw!(#[doc = "Application Program Status Register"] apsr);
sro!(#[doc = "Interrupt Program Status Register"] ipsr);
sro!(#[doc = "Exception Program Status Register"] epsr);
srw!(#[doc = "Priority Mask Register"] primask);
srw!(#[doc = "Fault Mask Register"] faultmask);
srw!(#[doc = "Base Priority Mask Register"] basepri);
srw!(#[doc = "Base Priority Mask Register"] basepri_max);
srw!(#[doc = "Control Register"] control);
