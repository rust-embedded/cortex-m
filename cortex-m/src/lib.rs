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
//! ## `critical-section-single-core`
//!
//! This feature enables a [`critical-section`](https://github.com/rust-embedded/critical-section)
//! implementation suitable for single-core targets, based on disabling interrupts globally.
//!
//! It is **unsound** to enable it on multi-core targets or for code running in unprivileged mode,
//! and may cause functional problems in systems where some interrupts must not be disabled
//! or critical sections are managed as part of an RTOS. In these cases, you should use
//! a target-specific implementation instead, typically provided by a HAL or RTOS crate.
//!
//! ## `cm7-r0p1`
//!
//! This feature enables workarounds for errata found on Cortex-M7 chips with revision r0p1. Some
//! functions in this crate only work correctly on those chips if this Cargo feature is enabled
//! (the functions are documented accordingly).
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.60 and up. It *might*
//! compile with older versions but that may change in any new patch release.

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
// Don't warn about feature(asm) being stable on Rust >= 1.59.0
#![allow(stable_features)]

#[macro_use]
mod macros;

pub mod asm;
#[cfg(armv8m)]
pub mod cmse;
#[cfg(not(armv6m))]
pub mod coprocessor;
pub mod delay;
pub mod interrupt;
#[cfg(all(not(armv6m), not(armv8m_base)))]
pub mod itm;
pub mod peripheral;
pub mod register;

pub use crate::peripheral::Peripherals;

#[cfg(all(cortex_m, feature = "critical-section-single-core"))]
mod critical_section;

/// Used to reexport items for use in macros. Do not use directly.
/// Not covered by semver guarantees.
#[doc(hidden)]
pub mod _export {
    pub use critical_section;
}
