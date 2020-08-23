//! Low level access to Cortex-M processors
//!
//! This crate provides:
//!
//! - Access to core peripherals like NVIC, SCB and SysTick.
//! - Access to core registers like CONTROL, MSP and PSR.
//! - Interrupt manipulation mechanisms
//! - Safe wrappers around Cortex-M specific instructions like `bkpt`
//!
//! # Optional features
//!
//! ## `inline-asm`
//!
//! When this feature is enabled the implementation of all the functions inside the `asm` and
//! `register` modules use inline assembly (`llvm_asm!`) instead of external assembly (FFI into separate
//! assembly files pre-compiled using `arm-none-eabi-gcc`). The advantages of enabling `inline-asm`
//! are:
//!
//! - Reduced overhead. FFI eliminates the possibility of inlining so all operations include a
//! function call overhead when `inline-asm` is not enabled.
//!
//! - Some of the `register` API only becomes available only when `inline-asm` is enabled. Check the
//! API docs for details.
//!
//! The disadvantage is that `inline-asm` requires a nightly toolchain.
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.31 and up. It *might*
//! compile with older versions but that may change in any new patch release.

#![cfg_attr(feature = "inline-asm", feature(llvm_asm))]
#![deny(missing_docs)]
#![no_std]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]
// Prevent clippy from complaining about empty match expression that are used for cfg gating.
#![allow(clippy::match_single_binding)]

// This makes clippy warn about public functions which are not #[inline].
//
// Almost all functions in this crate result in trivial or even no assembly.
// These functions should be #[inline].
//
// If you do add a function that's not supposed to be #[inline], you can add
// #[allow(clippy::missing_inline_in_public_items)] in front of it to add an
// exception to clippy's rules.
//
// This should be done in case of:
//  - A function containing non-trivial logic (such as itm::write_all); or
//  - A generated #[derive(Debug)] function (in which case the attribute needs
//    to be applied to the struct).
#![deny(clippy::missing_inline_in_public_items)]

extern crate bare_metal;
extern crate volatile_register;

#[macro_use]
mod macros;

pub mod asm;
#[cfg(armv8m)]
pub mod cmse;
pub mod delay;
pub mod interrupt;
#[cfg(all(not(armv6m), not(armv8m_base)))]
pub mod itm;
pub mod peripheral;
pub mod register;

pub use crate::peripheral::Peripherals;
