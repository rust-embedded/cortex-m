//! Low level access to Cortex-M processors
//!
//! This crate provides:
//!
//! - Access to core peripherals like NVIC, SCB and SysTick.
//! - Access to core registers like CONTROL, MSP and PSR.
//! - Interrupt manipulation mechanisms
//! - Safe wrappers around Cortex-M specific instructions like `bkpt`
//!
//! # Requirements
//!
//! To use this crate on the stable or beta channel `arm-none-eabi-gcc` needs to be installed and
//! available in your `$PATH`.
//!
//! # Optional features
//!
//! ## `inline-asm`
//!
//! When this feature is enabled the implementation of all the functions inside the `asm` and
//! `register` modules use inline assembly (`asm!`) instead of external assembly (FFI into separate
//! assembly files compiled using `arm-none-eabi-gcc`). The advantages the enabling `inline-asm`
//! are:
//!
//! - Reduced overhead. FFI eliminates the possibility of inlining so all operations include a
//! function call overhead when `inline-asm` is not enabled.
//!
//! - `arm-none-eabi-gcc` is not required for building this crate.
//!
//! - Some of the `register` API only becomes available only when `inline-asm` is enabled. Check the
//! API docs for details.
//!
//! The disadvantage is that `inline-asm` requires a nightly toolchain.

#![cfg_attr(feature = "inline-asm", feature(asm))]
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate aligned;
extern crate bare_metal;
extern crate volatile_register;

#[macro_use]
mod macros;

pub mod asm;
pub mod interrupt;
// NOTE(target_arch = "x86_64") is used throughout this crate for documentation purposes
#[cfg(any(armv7m, target_arch = "x86_64"))]
pub mod itm;
pub mod peripheral;
pub mod register;

pub use peripheral::Peripherals;
