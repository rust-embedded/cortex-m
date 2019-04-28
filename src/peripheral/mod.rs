//! Core peripherals
//!
//! # API
//!
//! To use (most of) the peripheral API first you must get an *instance* of the peripheral. All the
//! core peripherals are modeled as singletons (there can only ever be, at most, one instance of any
//! one of them at any given point in time) and the only way to get an instance of them is through
//! the [`Peripherals::take`](struct.Peripherals.html#method.take) method.
//!
//! ``` no_run
//! extern crate cortex_m;
//!
//! use cortex_m::peripheral::Peripherals;
//!
//! fn main() {
//!     let mut peripherals = Peripherals::take().unwrap();
//!     peripherals.DWT.enable_cycle_counter();
//! }
//! ```
//!
//! This method can only be successfully called *once* -- this is why the method returns an
//! `Option`. Subsequent calls to the method will result in a `None` value being returned.
//!
//! ``` no_run
//! extern crate cortex_m;
//!
//! use cortex_m::peripheral::Peripherals;
//!
//! fn main() {
//!     let ok = Peripherals::take().unwrap();
//!     let panics = Peripherals::take().unwrap();
//! }
//! ```
//! A part of the peripheral API doesn't require access to a peripheral instance. This part of the
//! API is provided as static methods on the peripheral types. One example is the
//! [`DWT::get_cycle_count`](struct.DWT.html#method.get_cycle_count) method.
//!
//! ``` no_run
//! extern crate cortex_m;
//!
//! use cortex_m::peripheral::{DWT, Peripherals};
//!
//! fn main() {
//!     {
//!         let mut peripherals = Peripherals::take().unwrap();
//!         peripherals.DWT.enable_cycle_counter();
//!     } // all the peripheral singletons are destroyed here
//!
//!     // but this method can be called without a DWT instance
//!     let cyccnt = DWT::get_cycle_count();
//! }
//! ```
//!
//! The singleton property can be *unsafely* bypassed using the `ptr` static method which is
//! available on all the peripheral types. This method is a useful building block for implementing
//! safe higher level abstractions.
//!
//! ``` no_run
//! extern crate cortex_m;
//!
//! use cortex_m::peripheral::{DWT, Peripherals};
//!
//! fn main() {
//!     {
//!         let mut peripherals = Peripherals::take().unwrap();
//!         peripherals.DWT.enable_cycle_counter();
//!     } // all the peripheral singletons are destroyed here
//!
//!     // actually safe because this is an atomic read with no side effects
//!     let cyccnt = unsafe { (*DWT::ptr()).cyccnt.read() };
//! }
//! ```
//!
//! # References
//!
//! - ARMv7-M Architecture Reference Manual (Issue E.b) - Chapter B3

// TODO stand-alone registers: ICTR, ACTLR and STIR


use core::marker::PhantomData;
use core::ops;

use interrupt;

pub mod dwt;
pub mod nvic;
pub mod scb;

#[cfg(not(armv6m))]
pub use cortex_m_0_6::peripheral::{cbp, fpb, itm, tpiu};
// NOTE(target_arch) is for documentation purposes
#[cfg(any(has_fpu, target_arch = "x86_64"))]
pub use cortex_m_0_6::peripheral::fpu;
pub use cortex_m_0_6::peripheral::{cpuid, dcb, mpu, syst};

#[cfg(test)]
mod test;

// NOTE the `PhantomData` used in the peripherals proxy is to make them `Send` but *not* `Sync`

/// Core peripherals
#[allow(non_snake_case)]
pub struct Peripherals {
    /// Cache and branch predictor maintenance operations (not present on Cortex-M0 variants)
    pub CBP: CBP,

    /// CPUID
    pub CPUID: CPUID,

    /// Debug Control Block
    pub DCB: DCB,

    /// Data Watchpoint and Trace unit
    pub DWT: DWT,

    /// Flash Patch and Breakpoint unit (not present on Cortex-M0 variants)
    pub FPB: FPB,

    /// Floating Point Unit (only present on `thumbv7em-none-eabihf`)
    pub FPU: FPU,

    /// Instrumentation Trace Macrocell (not present on Cortex-M0 variants)
    pub ITM: ITM,

    /// Memory Protection Unit
    pub MPU: MPU,

    /// Nested Vector Interrupt Controller
    pub NVIC: NVIC,

    /// System Control Block
    pub SCB: SCB,

    /// SysTick: System Timer
    pub SYST: SYST,

    /// Trace Port Interface Unit (not present on Cortex-M0 variants)
    pub TPIU: TPIU,
}

// Re-use the CORE_PERIPHERALS static from cortex-m v0.6.0 to allow interoperation
extern "C" {
    static mut CORE_PERIPHERALS: bool;
}

impl Peripherals {
    /// Returns all the core peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        interrupt::free(|_| {
            if unsafe { CORE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }

    /// Unchecked version of `Peripherals::take`
    pub unsafe fn steal() -> Self {
        debug_assert!(!CORE_PERIPHERALS);

        CORE_PERIPHERALS = true;
        core::mem::transmute(())
    }
}

pub use cortex_m_0_6::peripheral::{CBP, CPUID, DCB, FPB, FPU, ITM, MPU, SYST, TPIU};

/// Data Watchpoint and Trace unit
pub struct DWT {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for DWT {}

impl DWT {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const dwt::RegisterBlock {
        0xE000_1000 as *const _
    }
}

impl ops::Deref for DWT {
    type Target = self::dwt::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Nested Vector Interrupt Controller
pub struct NVIC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for NVIC {}

impl NVIC {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const nvic::RegisterBlock {
        0xE000_E100 as *const _
    }
}

impl ops::Deref for NVIC {
    type Target = self::nvic::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// System Control Block
pub struct SCB {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SCB {}

impl SCB {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const scb::RegisterBlock {
        0xE000_ED04 as *const _
    }
}

impl ops::Deref for SCB {
    type Target = self::scb::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}
