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
//! The rest of registers (see list below) can be accessed in either, PRIVILEGED or UNPRIVILEGED,
//! mode.
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

pub mod apsr;
#[cfg(not(thumbv6m))]
pub mod basepri;
#[cfg(not(thumbv6m))]
pub mod basepri_max;
pub mod control;
#[cfg(not(thumbv6m))]
pub mod faultmask;
pub mod lr;
pub mod msp;
pub mod pc;
pub mod primask;
pub mod psp;
