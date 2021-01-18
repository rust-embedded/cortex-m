#![allow(clippy::needless_doctest_main)]
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

use crate::interrupt;

#[cfg(not(armv6m))]
pub mod cbp;
#[cfg(armv8m_base)]
pub mod itm;
pub mod mpu;
pub mod nvic;
pub mod scb;

pub use cortex_m_0_7::peripheral::{cpuid, dcb, dwt, syst};
#[cfg(not(armv6m))]
pub use cortex_m_0_7::peripheral::{fpb, tpiu};
#[cfg(any(has_fpu, target_arch="x86_64"))]
pub use cortex_m_0_7::peripheral::fpu;

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

// NOTE: CORE_PERIPHERALS removed because this crate deliberately allows linking to other cortex-m
// versions by proxying calls to take() and steal() through cortex-m 0.7.
// Since TAKEN is no longer no_mangle nor public we can't set it ourselves.

impl Peripherals {
    /// Returns all the core peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        interrupt::free(|_| match cortex_m_0_7::peripheral::Peripherals::take() {
            Some(_) => { Some(unsafe { Peripherals::steal() }) },
            None    => None,
        })
    }

    /// Unchecked version of `Peripherals::take`
    #[inline]
    pub unsafe fn steal() -> Self {
        // Ensure peripherals are marked as taken.
        cortex_m_0_7::peripheral::Peripherals::steal();

        // We can't create the imported types like CPUID because
        // their _marker field is private, but we can create an
        // entire Peripherals out of thin air because all the
        // types are zero-sized.
        core::mem::transmute(())
    }
}

/// Cache and branch predictor maintenance operations
pub struct CBP {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for CBP {}

#[cfg(not(armv6m))]
impl CBP {
    #[inline(always)]
    pub(crate) unsafe fn new() -> Self {
        CBP {
            _marker: PhantomData,
        }
    }

    /// Returns a pointer to the register block
    #[inline(always)]
    pub fn ptr() -> *const self::cbp::RegisterBlock {
        0xE000_EF50 as *const _
    }
}

#[cfg(not(armv6m))]
impl ops::Deref for CBP {
    type Target = self::cbp::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

pub use cortex_m_0_7::peripheral::{CPUID, DCB, DWT, FPB, FPU, SYST, TPIU};

#[cfg(not(armv8m_base))]
pub use cortex_m_0_7::peripheral::ITM;

#[cfg(armv8m_base)]
/// Instrumentation Trace Macrocell
pub struct ITM {
    _marker: PhantomData<*const ()>,
}

#[cfg(armv8m_base)]
unsafe impl Send for ITM {}

#[cfg(armv8m_base)]
impl ITM {
    /// Returns a pointer to the register block
    #[inline(always)]
    pub fn ptr() -> *mut itm::RegisterBlock {
        0xE000_0000 as *mut _
    }
}

#[cfg(armv8m_base)]
impl ops::Deref for ITM {
    type Target = self::itm::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

#[cfg(armv8m_base)]
impl ops::DerefMut for ITM {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *Self::ptr() }
    }
}

/// Memory Protection Unit
pub struct MPU {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for MPU {}

impl MPU {
    /// Returns a pointer to the register block
    #[inline(always)]
    pub fn ptr() -> *const mpu::RegisterBlock {
        0xE000_ED90 as *const _
    }
}

impl ops::Deref for MPU {
    type Target = self::mpu::RegisterBlock;

    #[inline(always)]
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
    #[inline(always)]
    pub fn ptr() -> *const nvic::RegisterBlock {
        0xE000_E100 as *const _
    }
}

impl ops::Deref for NVIC {
    type Target = self::nvic::RegisterBlock;

    #[inline(always)]
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
    #[inline(always)]
    pub fn ptr() -> *const scb::RegisterBlock {
        0xE000_ED04 as *const _
    }
}

impl ops::Deref for SCB {
    type Target = self::scb::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}
