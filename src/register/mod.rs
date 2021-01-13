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

#[cfg(all(not(armv6m), not(armv8m_base)))]
pub use cortex_m_0_7::register::basepri;

#[cfg(armv8m_base)]
#[deprecated(
    since = "0.6.2",
    note = "basepri is unavailable on thumbv8.base, and will be removed in the next release"
)]
pub mod basepri;

#[cfg(all(not(armv6m), not(armv8m_base)))]
pub use cortex_m_0_7::register::basepri_max;

#[cfg(armv8m_base)]
#[deprecated(
    since = "0.6.2",
    note = "basepri is unavailable on thumbv8m.base, and will be removed in the next release"
)]
pub mod basepri_max;

pub use cortex_m_0_7::register::control;

#[cfg(all(not(armv6m), not(armv8m_base)))]
pub use cortex_m_0_7::register::faultmask;

#[cfg(armv8m_base)]
#[deprecated(
    since = "0.6.2",
    note = "faultmask is unavailable on thumbv8m.base, and will be removed in the next release"
)]
pub mod faultmask;

pub use cortex_m_0_7::register::{msp, primask, psp};

#[cfg(armv8m_main)]
pub use cortex_m_0_7::register::{msplim, psplim};

// Accessing these registers requires inline assembly because their contents are tied to the current
// stack frame
#[cfg(feature = "inline-asm")]
pub use cortex_m_0_7::register::{apsr, lr, pc};

#[cfg(target_arch = "x86_64")]
pub mod apsr;

#[cfg(target_arch = "x86_64")]
pub mod lr;

#[cfg(target_arch = "x86_64")]
pub mod pc;
