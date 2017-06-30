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

/// Cache and branch predictor maintenance operations
#[cfg(armv7m)]
pub const CBP: Peripheral<Cbp> = unsafe { Peripheral::new(0xE000_EF50) };

// TODO stand-alone registers: ICTR, ACTLR and STIR

/// A peripheral
#[derive(Debug)]
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
    #[cfg(any(armv7m, test))]
    pub clidr: RO<u32>,
    /// Cache Type
    #[cfg(any(armv7m, test))]
    pub ctr: RO<u32>,
    /// Cache Size ID
    #[cfg(any(armv7m, test))]
    pub ccsidr: RO<u32>,
    /// Cache Size Selection
    #[cfg(any(armv7m, test))]
    pub csselr: RW<u32>,
}

/// Type of cache to select on CSSELR writes.
#[cfg(armv7m)]
pub enum CsselrCacheType {
    /// Select DCache or unified cache
    DataOrUnified = 0,
    /// Select ICache
    Instruction = 1,
}

#[cfg(armv7m)]
impl Cpuid {
    /// Selects the current CCSIDR
    ///
    /// * `level`: the required cache level minus 1, e.g. 0 for L1, 1 for L2
    /// * `ind`: select instruction cache or data/unified cache
    ///
    /// `level` is masked to be between 0 and 7.
    pub fn select_cache(&self, level: u8, ind: CsselrCacheType) {
        const CSSELR_IND_POS: u32 = 0;
        const CSSELR_IND_MASK: u32 = 1 << CSSELR_IND_POS;
        const CSSELR_LEVEL_POS: u32 = 1;
        const CSSELR_LEVEL_MASK: u32 = 0x7 << CSSELR_LEVEL_POS;

        unsafe { self.csselr.write(
            (((level as u32) << CSSELR_LEVEL_POS) & CSSELR_LEVEL_MASK) |
            (((ind   as u32) <<   CSSELR_IND_POS) &   CSSELR_IND_MASK)
        )}
    }

    /// Returns the number of sets and ways in the selected cache
    pub fn cache_num_sets_ways(&self, level: u8, ind: CsselrCacheType) -> (u16, u16) {
        const CCSIDR_NUMSETS_POS: u32 = 13;
        const CCSIDR_NUMSETS_MASK: u32 = 0x7FFF << CCSIDR_NUMSETS_POS;
        const CCSIDR_ASSOCIATIVITY_POS: u32 = 3;
        const CCSIDR_ASSOCIATIVITY_MASK: u32 = 0x3FF << CCSIDR_ASSOCIATIVITY_POS;

        self.select_cache(level, ind);
        ::asm::dsb();
        let ccsidr = self.ccsidr.read();
        ((1 + ((ccsidr & CCSIDR_NUMSETS_MASK) >> CCSIDR_NUMSETS_POS)) as u16,
         (1 + ((ccsidr & CCSIDR_ASSOCIATIVITY_MASK) >> CCSIDR_ASSOCIATIVITY_POS)) as u16)
    }
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

impl Dwt {
    /// Enables the cycle counter
    pub fn enable_cycle_counter(&self) {
        unsafe { self.ctrl.modify(|r| r | 1) }
    }
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
    pub unsafe fn set_priority<I>(&self, interrupt: I, prio: u8)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        self.ipr[usize::from(nr)].write(prio)
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

/// FPU access mode
#[derive(Clone, Copy, Debug)]
pub enum FpuAccessMode {
    /// FPU is not accessible
    Disabled,
    /// FPU is accessible in Privileged and User mode
    Enabled,
    /// FPU is accessible in Privileged mode only
    Privileged,
}

const SCB_CPACR_FPU_MASK: u32 = 0b11_11 << 20;
const SCB_CPACR_FPU_ENABLE: u32 = 0b01_01 << 20;
const SCB_CPACR_FPU_USER: u32 = 0b10_10 << 20;

impl Scb {
    /// Gets FPU access mode
    pub fn fpu_access_mode(&self) -> FpuAccessMode {
        let cpacr = self.cpacr.read();
        if cpacr & SCB_CPACR_FPU_MASK ==
            SCB_CPACR_FPU_ENABLE | SCB_CPACR_FPU_USER
        {
            FpuAccessMode::Enabled
        } else if cpacr & SCB_CPACR_FPU_MASK == SCB_CPACR_FPU_ENABLE {
            FpuAccessMode::Privileged
        } else {
            FpuAccessMode::Disabled
        }
    }

    /// Sets FPU access mode
    pub fn set_fpu_access_mode(&self, mode: FpuAccessMode) {
        let mut cpacr = self.cpacr.read() & !SCB_CPACR_FPU_MASK;
        match mode {
            FpuAccessMode::Disabled => (),
            FpuAccessMode::Privileged => cpacr |= SCB_CPACR_FPU_ENABLE,
            FpuAccessMode::Enabled => {
                cpacr |= SCB_CPACR_FPU_ENABLE | SCB_CPACR_FPU_USER
            }
        }
        unsafe { self.cpacr.write(cpacr) }
    }

    /// Shorthand for `set_fpu_access_mode(FpuAccessMode::Enabled)`
    pub fn enable_fpu(&self) {
        self.set_fpu_access_mode(FpuAccessMode::Enabled)
    }

    /// Shorthand for `set_fpu_access_mode(FpuAccessMode::Disabled)`
    pub fn disable_fpu(&self) {
        self.set_fpu_access_mode(FpuAccessMode::Disabled)
    }
}

#[cfg(armv7m)]
mod scb_consts {
    pub const SCB_CCR_IC_MASK: u32 = (1<<17);
    pub const SCB_CCR_DC_MASK: u32 = (1<<16);
}

#[cfg(armv7m)]
use self::scb_consts::*;

#[cfg(armv7m)]
impl Scb {
    /// Enables I-Cache if currently disabled
    #[inline]
    pub fn enable_icache(&self) {
        // Don't do anything if ICache is already enabled
        if self.icache_enabled() {
            return;
        }

        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &mut *CBP.get() };

        // Invalidate I-Cache
        cbp.iciallu();

        // Enable I-Cache
        unsafe { self.ccr.modify(|r| r | SCB_CCR_IC_MASK) };

        ::asm::dsb();
        ::asm::isb();
    }

    /// Disables I-Cache if currently enabled
    #[inline]
    pub fn disable_icache(&self) {
        // Don't do anything if ICache is already disabled
        if !self.icache_enabled() {
            return;
        }

        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &mut *CBP.get() };

        // Disable I-Cache
        unsafe { self.ccr.modify(|r| r & !SCB_CCR_IC_MASK) };

        // Invalidate I-Cache
        cbp.iciallu();

        ::asm::dsb();
        ::asm::isb();
    }

    /// Returns whether the I-Cache is currently enabled
    #[inline]
    pub fn icache_enabled(&self) -> bool {
        ::asm::dsb();
        ::asm::isb();
        self.ccr.read() & SCB_CCR_IC_MASK == SCB_CCR_IC_MASK
    }

    /// Invalidates I-Cache
    #[inline]
    pub fn invalidate_icache(&self) {
        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &mut *CBP.get() };

        // Invalidate I-Cache
        cbp.iciallu();

        ::asm::dsb();
        ::asm::isb();
    }

    /// Enables D-cache if currently disabled
    #[inline]
    pub fn enable_dcache(&self, cpuid: &Cpuid) {
        // Don't do anything if DCache is already enabled
        if self.dcache_enabled() {
            return;
        }

        // Invalidate anything currently in the DCache
        self.invalidate_dcache(cpuid);

        // Now turn on the DCache
        unsafe { self.ccr.modify(|r| r | SCB_CCR_DC_MASK) };

        ::asm::dsb();
        ::asm::isb();
    }

    /// Disables D-cache if currently enabled
    #[inline]
    pub fn disable_dcache(&self, cpuid: &Cpuid) {
        // Don't do anything if DCache is already disabled
        if !self.dcache_enabled() {
            return;
        }

        // Turn off the DCache
        unsafe { self.ccr.modify(|r| r & !SCB_CCR_DC_MASK) };

        // Clean and invalidate whatever was left in it
        self.clean_invalidate_dcache(cpuid);
    }

    /// Returns whether the D-Cache is currently enabled
    #[inline]
    pub fn dcache_enabled(&self) -> bool {
        ::asm::dsb();
        ::asm::isb();
        self.ccr.read() & SCB_CCR_DC_MASK == SCB_CCR_DC_MASK
    }

    /// Invalidates D-cache
    ///
    /// Note that calling this while the dcache is enabled will probably wipe out your
    /// stack, depending on optimisations, breaking returning to the call point.
    /// It's used immediately before enabling the dcache, but not exported publicly.
    #[inline]
    fn invalidate_dcache(&self, cpuid: &Cpuid) {
        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &mut *CBP.get() };

        // Read number of sets and ways
        let (sets, ways) = cpuid.cache_num_sets_ways(0, CsselrCacheType::DataOrUnified);

        // Invalidate entire D-Cache
        for set in 0..sets {
            for way in 0..ways {
                cbp.dcisw(set, way);
            }
        }

        ::asm::dsb();
        ::asm::isb();
    }

    /// Cleans D-cache
    #[inline]
    pub fn clean_dcache(&self, cpuid: &Cpuid) {
        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &mut *CBP.get() };

        // Read number of sets and ways
        let (sets, ways) = cpuid.cache_num_sets_ways(0, CsselrCacheType::DataOrUnified);

        for set in 0..sets {
            for way in 0..ways {
                cbp.dccsw(set, way);
            }
        }

        ::asm::dsb();
        ::asm::isb();
    }

    /// Cleans and invalidates D-cache
    #[inline]
    pub fn clean_invalidate_dcache(&self, cpuid: &Cpuid) {
        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &mut *CBP.get() };

        // Read number of sets and ways
        let (sets, ways) = cpuid.cache_num_sets_ways(0, CsselrCacheType::DataOrUnified);

        for set in 0..sets {
            for way in 0..ways {
                cbp.dccisw(set, way);
            }
        }

        ::asm::dsb();
        ::asm::isb();
    }

    /// Invalidates D-cache by address
    ///
    /// `addr`: the address to invalidate
    /// `size`: size of the memory block, in number of bytes
    ///
    /// Invalidates cache starting from the lowest 32-byte aligned address represented by `addr`,
    /// in blocks of 32 bytes until at least `size` bytes have been invalidated.
    #[inline]
    pub fn invalidate_dcache_by_address(&self, addr: usize, size: usize) {
        // No-op zero sized operations
        if size == 0 {
            return;
        }

        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &mut *CBP.get() };

        ::asm::dsb();

        // Cache lines are fixed to 32 bit on Cortex-M7 and not present in earlier Cortex-M
        const LINESIZE: usize = 32;
        let num_lines = ((size - 1)/LINESIZE) + 1;

        let mut addr = addr & 0xFFFF_FFE0;

        for _ in 0..num_lines {
            cbp.dcimvac(addr as u32);
            addr += LINESIZE;
        }

        ::asm::dsb();
        ::asm::isb();
    }

    /// Cleans D-cache by address
    ///
    /// `addr`: the address to clean
    /// `size`: size of the memory block, in number of bytes
    ///
    /// Cleans cache starting from the lowest 32-byte aligned address represented by `addr`,
    /// in blocks of 32 bytes until at least `size` bytes have been cleaned.
    #[inline]
    pub fn clean_dcache_by_address(&self, addr: usize, size: usize) {
        // No-op zero sized operations
        if size == 0 {
            return;
        }

        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &mut *CBP.get() };

        ::asm::dsb();

        // Cache lines are fixed to 32 bit on Cortex-M7 and not present in earlier Cortex-M
        const LINESIZE: usize = 32;
        let num_lines = ((size - 1)/LINESIZE) + 1;

        let mut addr = addr & 0xFFFF_FFE0;

        for _ in 0..num_lines {
            cbp.dccmvac(addr as u32);
            addr += LINESIZE;
        }

        ::asm::dsb();
        ::asm::isb();
    }

    /// Cleans and invalidates D-cache by address
    ///
    /// `addr`: the address to clean and invalidate
    /// `size`: size of the memory block, in number of bytes
    ///
    /// Cleans and invalidates cache starting from the lowest 32-byte aligned address represented
    /// by `addr`, in blocks of 32 bytes until at least `size` bytes have been cleaned and
    /// invalidated.
    #[inline]
    pub fn clean_invalidate_dcache_by_address(&self, addr: usize, size: usize) {
        // No-op zero sized operations
        if size == 0 {
            return;
        }

        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &mut *CBP.get() };

        ::asm::dsb();

        // Cache lines are fixed to 32 bit on Cortex-M7 and not present in earlier Cortex-M
        const LINESIZE: usize = 32;
        let num_lines = ((size - 1)/LINESIZE) + 1;

        let mut addr = addr & 0xFFFF_FFE0;

        for _ in 0..num_lines {
            cbp.dccimvac(addr as u32);
            addr += LINESIZE;
        }

        ::asm::dsb();
        ::asm::isb();
    }
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

/// SysTick clock source
#[derive(Clone, Copy, Debug)]
pub enum SystClkSource {
    /// Core-provided clock
    Core,
    /// External reference clock
    External,
}

const SYST_COUNTER_MASK: u32 = 0x00ffffff;

const SYST_CSR_ENABLE: u32 = 1 << 0;
const SYST_CSR_TICKINT: u32 = 1 << 1;
const SYST_CSR_CLKSOURCE: u32 = 1 << 2;
const SYST_CSR_COUNTFLAG: u32 = 1 << 16;

const SYST_CALIB_SKEW: u32 = 1 << 30;
const SYST_CALIB_NOREF: u32 = 1 << 31;

impl Syst {
    /// Checks if counter is enabled
    pub fn is_counter_enabled(&self) -> bool {
        self.csr.read() & SYST_CSR_ENABLE != 0
    }

    /// Enables counter
    pub fn enable_counter(&self) {
        unsafe { self.csr.modify(|v| v | SYST_CSR_ENABLE) }
    }

    /// Disables counter
    pub fn disable_counter(&self) {
        unsafe { self.csr.modify(|v| v & !SYST_CSR_ENABLE) }
    }

    /// Checks if SysTick interrupt is enabled
    pub fn is_interrupt_enabled(&self) -> bool {
        self.csr.read() & SYST_CSR_TICKINT != 0
    }

    /// Enables SysTick interrupt
    pub fn enable_interrupt(&self) {
        unsafe { self.csr.modify(|v| v | SYST_CSR_TICKINT) }
    }

    /// Disables SysTick interrupt
    pub fn disable_interrupt(&self) {
        unsafe { self.csr.modify(|v| v & !SYST_CSR_TICKINT) }
    }

    /// Gets clock source
    pub fn get_clock_source(&self) -> SystClkSource {
        let clk_source_bit = self.csr.read() & SYST_CSR_CLKSOURCE != 0;
        match clk_source_bit {
            false => SystClkSource::External,
            true => SystClkSource::Core,
        }
    }

    /// Sets clock source
    pub fn set_clock_source(&self, clk_source: SystClkSource) {
        match clk_source {
            SystClkSource::External => unsafe {
                self.csr.modify(|v| v & !SYST_CSR_CLKSOURCE)
            },
            SystClkSource::Core => unsafe {
                self.csr.modify(|v| v | SYST_CSR_CLKSOURCE)
            },
        }
    }

    /// Checks if the counter wrapped (underflowed) since the last check
    pub fn has_wrapped(&self) -> bool {
        self.csr.read() & SYST_CSR_COUNTFLAG != 0
    }

    /// Gets reload value
    pub fn get_reload(&self) -> u32 {
        self.rvr.read()
    }

    /// Sets reload value
    ///
    /// Valid values are between `1` and `0x00ffffff`.
    pub fn set_reload(&self, value: u32) {
        unsafe { self.rvr.write(value) }
    }

    /// Gets current value
    pub fn get_current(&self) -> u32 {
        self.cvr.read()
    }

    /// Clears current value to 0
    ///
    /// After calling `clear_current()`, the next call to `has_wrapped()`
    /// will return `false`.
    pub fn clear_current(&self) {
        unsafe { self.cvr.write(0) }
    }

    /// Returns the reload value with which the counter would wrap once per 10
    /// ms
    ///
    /// Returns `0` if the value is not known (e.g. because the clock can
    /// change dynamically).
    pub fn get_ticks_per_10ms(&self) -> u32 {
        self.calib.read() & SYST_COUNTER_MASK
    }

    /// Checks if the calibration value is precise
    ///
    /// Returns `false` if using the reload value returned by
    /// `get_ticks_per_10ms()` may result in a period significantly deviating
    /// from 10 ms.
    pub fn is_precise(&self) -> bool {
        self.calib.read() & SYST_CALIB_SKEW == 0
    }

    /// Checks if an external reference clock is available
    pub fn has_reference_clock(&self) -> bool {
        self.calib.read() & SYST_CALIB_NOREF == 0
    }
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

/// Cache and branch predictor maintenance operations register block
#[repr(C)]
#[cfg(armv7m)]
pub struct Cbp {
    /// I-cache invalidate all to PoU
    pub iciallu: WO<u32>,
    reserved0: u32,
    /// I-cache invalidate by MVA to PoU
    pub icimvau: WO<u32>,
    /// D-cache invalidate by MVA to PoC
    pub dcimvac: WO<u32>,
    /// D-cache invalidate by set-way
    pub dcisw: WO<u32>,
    /// D-cache clean by MVA to PoU
    pub dccmvau: WO<u32>,
    /// D-cache clean by MVA to PoC
    pub dccmvac: WO<u32>,
    /// D-cache clean by set-way
    pub dccsw: WO<u32>,
    /// D-cache clean and invalidate by MVA to PoC
    pub dccimvac: WO<u32>,
    /// D-cache clean and invalidate by set-way
    pub dccisw: WO<u32>,
    /// Branch predictor invalidate all
    pub bpiall: WO<u32>,
}

#[cfg(armv7m)]
mod cbp_consts {
    pub const CBP_SW_WAY_POS: u32 = 30;
    pub const CBP_SW_WAY_MASK: u32 = 0x3 << CBP_SW_WAY_POS;
    pub const CBP_SW_SET_POS: u32 = 5;
    pub const CBP_SW_SET_MASK: u32 = 0x1FF << CBP_SW_SET_POS;
}

#[cfg(armv7m)]
use self::cbp_consts::*;

#[cfg(armv7m)]
impl Cbp {
    /// I-cache invalidate all to PoU
    #[inline(always)]
    pub fn iciallu(&self) {
        unsafe { self.iciallu.write(0); }
    }

    /// I-cache invalidate by MVA to PoU
    #[inline(always)]
    pub fn icimvau(&self, mva: u32) {
        unsafe { self.icimvau.write(mva); }
    }

    /// D-cache invalidate by MVA to PoC
    #[inline(always)]
    pub fn dcimvac(&self, mva: u32) {
        unsafe { self.dcimvac.write(mva); }
    }

    /// D-cache invalidate by set-way
    ///
    /// `set` is masked to be between 0 and 3, and `way` between 0 and 511.
    #[inline(always)]
    pub fn dcisw(&self, set: u16, way: u16) {
        // The ARMv7-M Architecture Reference Manual, as of Revision E.b, says these set/way
        // operations have a register data format which depends on the implementation's
        // associativity and number of sets. Specifically the 'way' and 'set' fields have
        // offsets 32-log2(ASSOCIATIVITY) and log2(LINELEN) respectively.
        //
        // However, in Cortex-M7 devices, these offsets are fixed at 30 and 5, as per the Cortex-M7
        // Generic User Guide section 4.8.3. Since no other ARMv7-M implementations except the
        // Cortex-M7 have a DCACHE or ICACHE at all, it seems safe to do the same thing as the
        // CMSIS-Core implementation and use fixed values.
        unsafe { self.dcisw.write(
            (((way as u32) & (CBP_SW_WAY_MASK >> CBP_SW_WAY_POS)) << CBP_SW_WAY_POS) |
            (((set as u32) & (CBP_SW_SET_MASK >> CBP_SW_SET_POS)) << CBP_SW_SET_POS));
        }
    }

    /// D-cache clean by MVA to PoU
    #[inline(always)]
    pub fn dccmvau(&self, mva: u32) {
        unsafe { self.dccmvau.write(mva); }
    }

    /// D-cache clean by MVA to PoC
    #[inline(always)]
    pub fn dccmvac(&self, mva: u32) {
        unsafe { self.dccmvac.write(mva); }
    }

    /// D-cache clean by set-way
    ///
    /// `set` is masked to be between 0 and 3, and `way` between 0 and 511.
    #[inline(always)]
    pub fn dccsw(&self, set: u16, way: u16) {
        // See comment for dcisw() about the format here
        unsafe { self.dccsw.write(
            (((way as u32) & (CBP_SW_WAY_MASK >> CBP_SW_WAY_POS)) << CBP_SW_WAY_POS) |
            (((set as u32) & (CBP_SW_SET_MASK >> CBP_SW_SET_POS)) << CBP_SW_SET_POS));
        }
    }

    /// D-cache clean and invalidate by MVA to PoC
    #[inline(always)]
    pub fn dccimvac(&self, mva: u32) {
        unsafe { self.dccimvac.write(mva); }
    }

    /// D-cache clean and invalidate by set-way
    ///
    /// `set` is masked to be between 0 and 3, and `way` between 0 and 511.
    #[inline(always)]
    pub fn dccisw(&self, set: u16, way: u16) {
        // See comment for dcisw() about the format here
        unsafe { self.dccisw.write(
            (((way as u32) & (CBP_SW_WAY_MASK >> CBP_SW_WAY_POS)) << CBP_SW_WAY_POS) |
            (((set as u32) & (CBP_SW_SET_MASK >> CBP_SW_SET_POS)) << CBP_SW_SET_POS));
        }
    }

    /// Branch predictor invalidate all
    #[inline(always)]
    pub fn bpiall(&self) {
        unsafe { self.bpiall.write(0); }
    }
}
