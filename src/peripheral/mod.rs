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
pub mod cpuid;
pub mod dcb;
pub mod dwt;
#[cfg(not(armv6m))]
pub mod fpb;
// NOTE(target_arch) is for documentation purposes
#[cfg(any(has_fpu, target_arch = "x86_64"))]
pub mod fpu;
#[cfg(not(armv6m))]
pub mod itm;
pub mod mpu;
pub mod nvic;
pub mod scb;
pub mod syst;
#[cfg(not(armv6m))]
pub mod tpiu;

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

// NOTE `no_mangle` is used here to prevent linking different minor versions of this crate as that
// would let you `take` the core peripherals more than once (one per minor version)
#[no_mangle]
static mut CORE_PERIPHERALS: bool = false;

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
        CORE_PERIPHERALS = true;

        Peripherals {
            CBP: CBP {
                _marker: PhantomData,
            },
            CPUID: CPUID {
                _marker: PhantomData,
            },
            DCB: DCB {
                _marker: PhantomData,
            },
            DWT: DWT {
                _marker: PhantomData,
            },
            FPB: FPB {
                _marker: PhantomData,
            },
            FPU: FPU {
                _marker: PhantomData,
            },
            ITM: ITM {
                _marker: PhantomData,
            },
            MPU: MPU {
                _marker: PhantomData,
            },
            NVIC: NVIC {
                _marker: PhantomData,
            },
            SCB: SCB {
                _marker: PhantomData,
            },
            SYST: SYST {
                _marker: PhantomData,
            },
            TPIU: TPIU {
                _marker: PhantomData,
            },
        }
    }
}

/// Cache and branch predictor maintenance operations
pub struct CBP {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for CBP {}

#[cfg(not(armv6m))]
impl CBP {
    pub(crate) unsafe fn new() -> Self {
        CBP {
            _marker: PhantomData,
        }
    }

    /// Returns a pointer to the register block
    pub fn ptr() -> *const self::cbp::RegisterBlock {
        0xE000_EF50 as *const _
    }
}

#[cfg(not(armv6m))]
impl ops::Deref for CBP {
    type Target = self::cbp::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// CPUID
pub struct CPUID {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for CPUID {}

impl CPUID {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const self::cpuid::RegisterBlock {
        0xE000_ED00 as *const _
    }
}

impl ops::Deref for CPUID {
    type Target = self::cpuid::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Debug Control Block
pub struct DCB {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for DCB {}

impl DCB {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const dcb::RegisterBlock {
        0xE000_EDF0 as *const _
    }
}

impl ops::Deref for DCB {
    type Target = self::dcb::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*DCB::ptr() }
    }
}

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

/// Flash Patch and Breakpoint unit
pub struct FPB {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for FPB {}

#[cfg(not(armv6m))]
impl FPB {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const fpb::RegisterBlock {
        0xE000_2000 as *const _
    }
}

#[cfg(not(armv6m))]
impl ops::Deref for FPB {
    type Target = self::fpb::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Floating Point Unit
pub struct FPU {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for FPU {}

#[cfg(any(has_fpu, target_arch = "x86_64"))]
impl FPU {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const fpu::RegisterBlock {
        0xE000_EF30 as *const _
    }
}

#[cfg(any(has_fpu, target_arch = "x86_64"))]
impl ops::Deref for FPU {
    type Target = self::fpu::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Instrumentation Trace Macrocell
pub struct ITM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ITM {}

#[cfg(not(armv6m))]
impl ITM {
    /// Returns a pointer to the register block
    pub fn ptr() -> *mut itm::RegisterBlock {
        0xE000_0000 as *mut _
    }
}

#[cfg(not(armv6m))]
impl ops::Deref for ITM {
    type Target = self::itm::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

#[cfg(not(armv6m))]
impl ops::DerefMut for ITM {
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
    pub fn ptr() -> *const mpu::RegisterBlock {
        0xE000_ED90 as *const _
    }
}

impl ops::Deref for MPU {
    type Target = self::mpu::RegisterBlock;

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
    /// Unsafely obtains an `SCB` instance from nothing.
    /// 
    /// # Safety
    /// 
    /// This is only safe to use when the returned `SCB` instance is used
    /// correctly without conflicting with any other existing `SCB` instances.
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

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

/// SysTick: System Timer
pub struct SYST {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SYST {}

impl SYST {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const syst::RegisterBlock {
        0xE000_E010 as *const _
    }
}

impl ops::Deref for SYST {
    type Target = self::syst::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Trace Port Interface Unit
pub struct TPIU {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TPIU {}

#[cfg(not(armv6m))]
impl TPIU {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const tpiu::RegisterBlock {
        0xE004_0000 as *const _
    }
}

#[cfg(not(armv6m))]
impl ops::Deref for TPIU {
    type Target = self::tpiu::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}
