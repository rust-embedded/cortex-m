//! Core peripherals
//!
//! # API
//!
//! To use (most of) the peripheral API first you must get an *instance* of the peripheral. All the
//! core peripherals are modeled as singletons (there can only ever be, at most, one instance of
//! them at any given point in time) and the only way to get an instance of them is through the
//! [`Peripherals::take`](struct.Peripherals.html#method.take) method.
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
//! A part of the peripheral API doesn't require access to a peripheral instance. This part of the
//! API is provided as static methods on the peripheral types. One example is the
//! [`DWT::cyccnt`](struct.DWT.html#method.cyccnt) method.
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
//! higher level and safe abstractions.
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

#![allow(private_no_mangle_statics)]

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use interrupt;

#[cfg(armv7m)]
pub mod cbp;
pub mod cpuid;
pub mod dcb;
pub mod dwt;
#[cfg(any(armv7m, test))]
pub mod fpb;
#[cfg(any(has_fpu, test))]
pub mod fpu;
#[cfg(any(armv7m, test))]
pub mod itm;
pub mod mpu;
pub mod nvic;
pub mod scb;
pub mod syst;
#[cfg(any(armv7m, test))]
pub mod tpiu;

#[cfg(test)]
mod test;

// NOTE the `PhantomData` used in the peripherals proxy is to make them `Send` but *not* `Sync`

/// Core peripherals
#[allow(non_snake_case)]
pub struct Peripherals {
    /// Cache and branch predictor maintenance operations
    #[cfg(armv7m)]
    pub CBP: CBP,
    /// CPUID
    pub CPUID: CPUID,
    /// Debug Control Block
    pub DCB: DCB,
    /// Data Watchpoint and Trace unit
    pub DWT: DWT,
    /// Flash Patch and Breakpoint unit
    #[cfg(armv7m)]
    pub FPB: FPB,
    /// Floating Point Unit
    #[cfg(has_fpu)]
    pub FPU: FPU,
    /// Instrumentation Trace Macrocell
    #[cfg(armv7m)]
    pub ITM: ITM,
    /// Memory Protection Unit
    pub MPU: MPU,
    /// Nested Vector Interrupt Controller
    pub NVIC: NVIC,
    /// System Control Block
    pub SCB: SCB,
    /// SysTick: System Timer
    pub SYST: SYST,
    /// Trace Port Interface Unit;
    #[cfg(armv7m)]
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
        debug_assert!(!CORE_PERIPHERALS);

        CORE_PERIPHERALS = true;

        Peripherals {
            #[cfg(armv7m)]
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
            #[cfg(armv7m)]
            FPB: FPB {
                _marker: PhantomData,
            },
            #[cfg(has_fpu)]
            FPU: FPU {
                _marker: PhantomData,
            },
            #[cfg(armv7m)]
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
            #[cfg(armv7m)]
            TPIU: TPIU {
                _marker: PhantomData,
            },
        }
    }
}

/// Cache and branch predictor maintenance operations
#[cfg(armv7m)]
pub struct CBP {
    _marker: PhantomData<*const ()>,
}

#[cfg(armv7m)]
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

#[cfg(armv7m)]
unsafe impl Send for CBP {}

#[cfg(armv7m)]
impl Deref for CBP {
    type Target = self::cbp::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// CPUID
pub struct CPUID {
    _marker: PhantomData<*const ()>,
}

impl CPUID {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const self::cpuid::RegisterBlock {
        0xE000_ED00 as *const _
    }
}

impl Deref for CPUID {
    type Target = self::cpuid::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Debug Control Block
pub struct DCB {
    _marker: PhantomData<*const ()>,
}

impl DCB {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const dcb::RegisterBlock {
        0xE000_EDF0 as *const _
    }
}

impl Deref for DCB {
    type Target = self::dcb::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*DCB::ptr() }
    }
}

/// Data Watchpoint and Trace unit
pub struct DWT {
    _marker: PhantomData<*const ()>,
}

impl DWT {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const dwt::RegisterBlock {
        0xE000_1000 as *const _
    }
}

impl Deref for DWT {
    type Target = self::dwt::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Flash Patch and Breakpoint unit
#[cfg(any(armv7m, test))]
pub struct FPB {
    _marker: PhantomData<*const ()>,
}

#[cfg(any(armv7m, test))]
impl FPB {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const fpb::RegisterBlock {
        0xE000_2000 as *const _
    }
}

#[cfg(armv7m)]
impl Deref for FPB {
    type Target = self::fpb::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Floating Point Unit
#[cfg(any(has_fpu, test))]
pub struct FPU {
    _marker: PhantomData<*const ()>,
}

#[cfg(any(has_fpu, test))]
impl FPU {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const fpu::RegisterBlock {
        0xE000_EF30 as *const _
    }
}

#[cfg(has_fpu)]
impl Deref for FPU {
    type Target = self::fpu::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Instrumentation Trace Macrocell
#[cfg(any(armv7m, test))]
pub struct ITM {
    _marker: PhantomData<*const ()>,
}

#[cfg(any(armv7m, test))]
impl ITM {
    /// Returns a pointer to the register block
    pub fn ptr() -> *mut itm::RegisterBlock {
        0xE000_0000 as *mut _
    }
}

#[cfg(armv7m)]
impl Deref for ITM {
    type Target = self::itm::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

#[cfg(armv7m)]
impl DerefMut for ITM {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *Self::ptr() }
    }
}

/// Memory Protection Unit
pub struct MPU {
    _marker: PhantomData<*const ()>,
}

impl MPU {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const mpu::RegisterBlock {
        0xE000_ED90 as *const _
    }
}

impl Deref for MPU {
    type Target = self::mpu::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Nested Vector Interrupt Controller
pub struct NVIC {
    _marker: PhantomData<*const ()>,
}

impl NVIC {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const nvic::RegisterBlock {
        0xE000_E100 as *const _
    }
}

impl Deref for NVIC {
    type Target = self::nvic::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// System Control Block
pub struct SCB {
    _marker: PhantomData<*const ()>,
}

impl SCB {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const scb::RegisterBlock {
        0xE000_ED04 as *const _
    }
}

impl Deref for SCB {
    type Target = self::scb::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// SysTick: System Timer
pub struct SYST {
    _marker: PhantomData<*const ()>,
}

impl SYST {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const syst::RegisterBlock {
        0xE000_E010 as *const _
    }
}

impl Deref for SYST {
    type Target = self::syst::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Trace Port Interface Unit;
#[cfg(any(armv7m, test))]
pub struct TPIU {
    _marker: PhantomData<*const ()>,
}

#[cfg(any(armv7m, test))]
impl TPIU {
    /// Returns a pointer to the register block
    pub fn ptr() -> *const tpiu::RegisterBlock {
        0xE004_0000 as *const _
    }
}

#[cfg(armv7m)]
impl Deref for TPIU {
    type Target = self::tpiu::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}
