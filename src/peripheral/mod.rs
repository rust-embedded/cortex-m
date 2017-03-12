//! Core peripherals
//!
//! # References
//!
//! - ARMv7-M Architecture Reference Manual (Issue E.b) - Chapter B3

use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ptr;

use volatile_register::{RO, RW, WO};

use interrupt::{CriticalSection, Nr};

#[cfg(test)]
mod test;

/// CPUID
pub const CPUID: Peripheral<Cpuid> = unsafe { Peripheral::new(0xE000_ED00) };

/// Debug Control Block
pub const DCB: Peripheral<Dcb> = unsafe { Peripheral::new(0xE000_EDF0) };

/// Data Watchpoint and Trace unit
pub const DWT: Peripheral<Dwt> = unsafe { Peripheral::new(0xE000_1000) };

/// Flash Patch and Breakpoint unit
pub const FPB: Peripheral<Fpb> = unsafe { Peripheral::new(0xE000_2000) };

/// Floating Point Unit
pub const FPU: Peripheral<Fpu> = unsafe { Peripheral::new(0xE000_EF30) };

/// Instrumentation Trace Macrocell
pub const ITM: Peripheral<Itm> = unsafe { Peripheral::new(0xE000_0000) };

/// Memory Protection Unit
pub const MPU: Peripheral<Mpu> = unsafe { Peripheral::new(0xE000_ED90) };

/// Nested Vector Interrupt Controller
pub const NVIC: Peripheral<Nvic> = unsafe { Peripheral::new(0xE000_E100) };

/// System Control Block
pub const SCB: Peripheral<Scb> = unsafe { Peripheral::new(0xE000_ED04) };

/// SysTick: System Timer
pub const SYST: Peripheral<Syst> = unsafe { Peripheral::new(0xE000_E010) };

/// Trace Port Interface Unit;
pub const TPIU: Peripheral<Tpiu> = unsafe { Peripheral::new(0xE004_0000) };

// TODO stand-alone registers: ICTR, ACTLR and STIR

/// A peripheral
pub struct Peripheral<T>
where
    T: 'static,
{
    address: usize,
    _marker: PhantomData<&'static mut T>,
}

impl<T> Peripheral<T> {
    /// Creates a new peripheral
    ///
    /// `address` is the base address of the register block
    pub const unsafe fn new(address: usize) -> Self {
        Peripheral {
            address: address,
            _marker: PhantomData,
        }
    }

    /// Borrows the peripheral for the duration of a critical section
    pub fn borrow<'cs>(&self, _ctxt: &'cs CriticalSection) -> &'cs T {
        unsafe { &*self.get() }
    }

    /// Returns a pointer to the register block
    pub fn get(&self) -> *mut T {
        self.address as *mut T
    }
}

/// CPUID register block
#[repr(C)]
pub struct Cpuid {
    /// CPUID base
    pub base: RO<u32>,
    reserved0: [u32; 15],
    /// Processor Feature
    pub pfr: [RO<u32>; 2],
    /// Debug Feature
    pub dfr: RO<u32>,
    /// Auxiliary Feature
    pub afr: RO<u32>,
    /// Memory Model Feature
    pub mmfr: [RO<u32>; 4],
    /// Instruction Set Attribute
    pub isar: [RO<u32>; 5],
    reserved1: u32,
    /// Cache Level ID
    pub clidr: RO<u32>,
    /// Cache Type
    pub ctr: RO<u32>,
    /// Cache Size ID
    pub ccsidr: RO<u32>,
    /// Cache Size Selection
    pub csselr: RO<u32>,
}

/// DCB register block
#[repr(C)]
pub struct Dcb {
    /// Debug Halting Control and Status
    pub dhcsr: RW<u32>,
    /// Debug Core Register Selector
    pub dcrsr: WO<u32>,
    /// Debug Core Register Data
    pub dcrdr: RW<u32>,
    /// Debug Exception and Monitor Control
    pub demcr: RW<u32>,
}

/// DWT register block
#[repr(C)]
pub struct Dwt {
    /// Control
    pub ctrl: RW<u32>,
    /// Cycle Count
    pub cyccnt: RW<u32>,
    /// CPI Count
    pub cpicnt: RW<u32>,
    /// Exception Overhead Count
    pub exccnt: RW<u32>,
    /// Sleep Count
    pub sleepcnt: RW<u32>,
    /// LSU Count
    pub lsucnt: RW<u32>,
    /// Folded-instruction Count
    pub foldcnt: RW<u32>,
    /// Program Counter Sample
    pub pcsr: RO<u32>,
    /// Comparators
    pub c: [Comparator; 16],
    reserved: [u32; 932],
    /// Lock Access
    pub lar: WO<u32>,
    /// Lock Status
    pub lsr: RO<u32>,
}

/// Comparator
#[repr(C)]
pub struct Comparator {
    /// Comparator
    pub comp: RW<u32>,
    /// Comparator Mask
    pub mask: RW<u32>,
    /// Comparator Function
    pub function: RW<u32>,
    reserved: u32,
}

/// FPB register block
#[repr(C)]
pub struct Fpb {
    /// Control
    pub ctrl: RW<u32>,
    /// Remap
    pub remap: RW<u32>,
    /// Comparator
    pub comp: [RW<u32>; 127],
    reserved: [u32; 875],
    /// Lock Access
    pub lar: WO<u32>,
    /// Lock Status
    pub lsr: RO<u32>,
}

/// FPU register block
#[repr(C)]
pub struct Fpu {
    reserved: u32,
    /// Floating Point Context Control
    pub fpccr: RW<u32>,
    /// Floating Point Context Address
    pub fpcar: RW<u32>,
    /// Floating Point Default Status Control
    pub fpdscr: RW<u32>,
    /// Media and FP Feature
    pub mvfr: [RO<u32>; 3],
}

/// ITM register block
#[repr(C)]
pub struct Itm {
    /// Stimulus Port
    pub stim: [Stim; 256],
    reserved0: [u32; 640],
    /// Trace Enable
    pub ter: [RW<u32>; 8],
    reserved1: [u32; 8],
    /// Trace Privilege
    pub tpr: RW<u32>,
    reserved2: [u32; 15],
    /// Trace Control
    pub tcr: RW<u32>,
    reserved3: [u32; 75],
    /// Lock Access
    pub lar: WO<u32>,
    /// Lock Status
    pub lsr: RO<u32>,
}

/// Stimulus Port
pub struct Stim {
    register: UnsafeCell<u32>,
}

impl Stim {
    /// Writes an `u8` payload into the stimulus port
    pub fn write_u8(&self, value: u8) {
        unsafe { ptr::write_volatile(self.register.get() as *mut u8, value) }
    }

    /// Writes an `u16` payload into the stimulus port
    pub fn write_u16(&self, value: u16) {
        unsafe { ptr::write_volatile(self.register.get() as *mut u16, value) }
    }

    /// Writes an `u32` payload into the stimulus port
    pub fn write_u32(&self, value: u32) {
        unsafe { ptr::write_volatile(self.register.get(), value) }
    }

    /// Returns `true` if the stimulus port is ready to accept more data
    pub fn is_fifo_ready(&self) -> bool {
        unsafe { ptr::read_volatile(self.register.get()) == 1 }
    }
}

/// MPU register block
#[repr(C)]
pub struct Mpu {
    /// Type
    pub _type: RO<u32>,
    /// Control
    pub ctrl: RW<u32>,
    /// Region Number
    pub rnr: RW<u32>,
    /// Region Base Address
    pub rbar: RW<u32>,
    /// Region Attribute and Size
    pub rasr: RW<u32>,
    /// Alias 1 of RBAR
    pub rbar_a1: RW<u32>,
    /// Alias 1 of RSAR
    pub rsar_a1: RW<u32>,
    /// Alias 2 of RBAR
    pub rbar_a2: RW<u32>,
    /// Alias 2 of RSAR
    pub rsar_a2: RW<u32>,
    /// Alias 3 of RBAR
    pub rbar_a3: RW<u32>,
    /// Alias 3 of RSAR
    pub rsar_a3: RW<u32>,
}

/// NVIC register block
#[repr(C)]
pub struct Nvic {
    /// Interrupt Set-Enable
    pub iser: [RW<u32>; 8],
    reserved0: [u32; 24],
    /// Interrupt Clear-Enable
    pub icer: [RW<u32>; 8],
    reserved1: [u32; 24],
    /// Interrupt Set-Pending
    pub ispr: [RW<u32>; 8],
    reserved2: [u32; 24],
    /// Interrupt Clear-Pending
    pub icpr: [RW<u32>; 8],
    reserved3: [u32; 24],
    /// Interrupt Active Bit
    pub iabr: [RO<u32>; 8],
    reserved4: [u32; 56],
    /// Interrupt Priority
    pub ipr: [RW<u8>; 240],
}

impl Nvic {
    /// Clears `interrupt`'s pending state
    pub fn clear_pending<I>(&self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.icpr[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Disables `interrupt`
    pub fn disable<I>(&self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.icer[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Enables `interrupt`
    pub fn enable<I>(&self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.iser[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Gets the "priority" of `interrupt`
    ///
    /// NOTE NVIC encodes priority in the highest bits of a byte so values like
    /// `1` and `2` have the same priority. Also for NVIC priorities, a lower
    /// value (e.g. `16`) has higher priority than a larger value (e.g. `32`).
    pub fn get_priority<I>(&self, interrupt: I) -> u8
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        self.ipr[usize::from(nr)].read()
    }

    /// Is `interrupt` active or pre-empted and stacked
    pub fn is_active<I>(&self, interrupt: I) -> bool
    where
        I: Nr,
    {
        let nr = interrupt.nr();
        let mask = 1 << (nr % 32);

        (self.iabr[usize::from(nr / 32)].read() & mask) == mask
    }

    /// Checks if `interrupt` is enabled
    pub fn is_enabled<I>(&self, interrupt: I) -> bool
    where
        I: Nr,
    {
        let nr = interrupt.nr();
        let mask = 1 << (nr % 32);

        (self.iser[usize::from(nr / 32)].read() & mask) == mask
    }

    /// Checks if `interrupt` is pending
    pub fn is_pending<I>(&self, interrupt: I) -> bool
    where
        I: Nr,
    {
        let nr = interrupt.nr();
        let mask = 1 << (nr % 32);

        (self.ispr[usize::from(nr / 32)].read() & mask) == mask
    }

    /// Forces `interrupt` into pending state
    pub fn set_pending<I>(&self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.ispr[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Sets the "priority" of `interrupt` to `prio`
    ///
    /// NOTE See `get_priority` method for an explanation of how NVIC priorities
    /// work.
    pub fn set_priority<I>(&self, interrupt: I, prio: u8)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.ipr[usize::from(nr)].write(prio) }
    }
}

/// SCB register block
#[repr(C)]
pub struct Scb {
    /// Interrupt Control and State
    pub icsr: RW<u32>,
    /// Vector Table Offset
    pub vtor: RW<u32>,
    /// Application Interrupt and Reset Control
    pub aircr: RW<u32>,
    /// System Control
    pub scr: RW<u32>,
    /// Configuration and Control
    pub ccr: RW<u32>,
    /// System Handler Priority
    pub shpr: [RW<u8>; 12],
    /// System Handler Control and State
    pub shpcrs: RW<u32>,
    /// Configurable Fault Status
    pub cfsr: RW<u32>,
    /// HardFault Status
    pub hfsr: RW<u32>,
    /// Debug Fault Status
    pub dfsr: RW<u32>,
    /// MemManage Fault Address
    pub mmar: RW<u32>,
    /// BusFault Address
    pub bfar: RW<u32>,
    /// Auxiliary Fault Status
    pub afsr: RW<u32>,
    reserved: [u32; 18],
    /// Coprocessor Access Control
    pub cpacr: RW<u32>,
}

/// SysTick register block
#[repr(C)]
pub struct Syst {
    /// Control and Status
    pub csr: RW<u32>,
    /// Reload Value
    pub rvr: RW<u32>,
    /// Current Value
    pub cvr: RW<u32>,
    /// Calibration Value
    pub calib: RO<u32>,
}

/// TPIU register block
#[repr(C)]
pub struct Tpiu {
    /// Supported Parallel Port Sizes
    pub sspsr: RO<u32>,
    /// Current Parallel Port Size
    pub cspsr: RW<u32>,
    reserved0: [u32; 2],
    /// Asynchronous Clock Prescaler
    pub acpr: RW<u32>,
    reserved1: [u32; 55],
    /// Selected Pin Control
    pub sppr: RW<u32>,
    reserved2: [u32; 943],
    /// Lock Access
    pub lar: WO<u32>,
    /// Lock Status
    pub lsr: RO<u32>,
    reserved3: [u32; 4],
    /// TPIU Type
    pub _type: RO<u32>,
}
