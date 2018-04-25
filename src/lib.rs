//! Low level access to Cortex-M processors
//!
//! This crate provides:
//!
//! - Access to core peripherals like NVIC, SCB and SysTick.
//! - Access to core registers like CONTROL, MSP and PSR.
//! - Interrupt manipulation mechanisms
//! - Safe wrappers around assembly instructions like `bkpt`

#![deny(missing_docs)]
#![deny(warnings)]
#![cfg_attr(feature = "inline-asm", feature(asm))]
#![no_std]

extern crate aligned;
extern crate bare_metal;
#[cfg(feature = "singleton")]
extern crate untagged_option;
extern crate volatile_register;

#[macro_use]
mod macros;

pub mod asm;
pub mod exception;
pub mod interrupt;
// NOTE(target_arch) is for documentation purposes
#[cfg(any(armv7m, target_arch = "x86_64"))]
pub mod itm;
pub mod peripheral;
pub mod register;

pub use peripheral::Peripherals;

#[cfg(feature = "singleton")]
#[doc(hidden)]
pub use untagged_option::UntaggedOption;
