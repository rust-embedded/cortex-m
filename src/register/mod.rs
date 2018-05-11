//! Processor core registers
//!
//! The following registers can only be accessed in PRIVILEGED mode:
//!
//! - BASEPRI
//! - CONTROL
//! - FAULTMASK
//! - MSP
//! - PRIMASK
//!
//! The rest of registers (see list below) can be accessed in either, PRIVILEGED
//! or UNPRIVILEGED, mode.
//!
//! - APSR
//! - LR
//! - PC
//! - PSP
//!
//! The following registers are NOT available on ARMv6-M devices
//! (`thumbv6m-none-eabi`):
//!
//! - BASEPRI
//! - FAULTMASK
//!
//! # References
//!
//! - Cortex-M* Devices Generic User Guide - Section 2.1.3 Core registers

#[cfg(not(armv6m))]
pub mod basepri;

#[cfg(not(armv6m))]
pub mod basepri_max;

pub mod control;

#[cfg(not(armv6m))]
pub mod faultmask;

pub mod msp;

pub mod primask;

pub mod psp;

// Accessing these registers requires inline assembly because their contents are tied to the current
// stack frame
#[cfg(any(feature = "inline-asm", target_arch = "x86_64"))]
pub mod apsr;

#[cfg(any(feature = "inline-asm", target_arch = "x86_64"))]
pub mod lr;

#[cfg(any(feature = "inline-asm", target_arch = "x86_64"))]
pub mod pc;
