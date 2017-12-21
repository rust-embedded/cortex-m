//! System Control Block

use volatile_register::RW;

#[cfg(armv7m)]
use super::CBP;
#[cfg(armv7m)]
use super::cpuid::{self, CsselrCacheType};

/// Register block
#[repr(C)]
pub struct RegisterBlock {
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
#[cfg(has_fpu)]
#[derive(Clone, Copy, Debug)]
pub enum FpuAccessMode {
    /// FPU is not accessible
    Disabled,
    /// FPU is accessible in Privileged and User mode
    Enabled,
    /// FPU is accessible in Privileged mode only
    Privileged,
}

#[cfg(has_fpu)]
mod fpu_consts {
    pub const SCB_CPACR_FPU_MASK: u32 = 0b11_11 << 20;
    pub const SCB_CPACR_FPU_ENABLE: u32 = 0b01_01 << 20;
    pub const SCB_CPACR_FPU_USER: u32 = 0b10_10 << 20;
}

#[cfg(has_fpu)]
use self::fpu_consts::*;

#[cfg(has_fpu)]
impl RegisterBlock {
    /// Gets FPU access mode
    pub fn fpu_access_mode(&self) -> FpuAccessMode {
        let cpacr = self.cpacr.read();
        if cpacr & SCB_CPACR_FPU_MASK == SCB_CPACR_FPU_ENABLE | SCB_CPACR_FPU_USER {
            FpuAccessMode::Enabled
        } else if cpacr & SCB_CPACR_FPU_MASK == SCB_CPACR_FPU_ENABLE {
            FpuAccessMode::Privileged
        } else {
            FpuAccessMode::Disabled
        }
    }

    /// Sets FPU access mode
    ///
    /// *IMPORTANT* Any function that runs fully or partly with the FPU disabled must *not* take any
    /// floating-point arguments or have any floating-point local variables. Because the compiler
    /// might inline such a function into a caller that does have floating-point arguments or
    /// variables, any such function must be also marked #[inline(never)].
    pub fn set_fpu_access_mode(&self, mode: FpuAccessMode) {
        let mut cpacr = self.cpacr.read() & !SCB_CPACR_FPU_MASK;
        match mode {
            FpuAccessMode::Disabled => (),
            FpuAccessMode::Privileged => cpacr |= SCB_CPACR_FPU_ENABLE,
            FpuAccessMode::Enabled => cpacr |= SCB_CPACR_FPU_ENABLE | SCB_CPACR_FPU_USER,
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
    pub const SCB_CCR_IC_MASK: u32 = (1 << 17);
    pub const SCB_CCR_DC_MASK: u32 = (1 << 16);
}

#[cfg(armv7m)]
use self::scb_consts::*;

#[cfg(armv7m)]
impl RegisterBlock {
    /// Enables I-Cache if currently disabled
    #[inline]
    pub fn enable_icache(&self) {
        // Don't do anything if ICache is already enabled
        if self.icache_enabled() {
            return;
        }

        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &*CBP::ptr() };

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
        let cbp = unsafe { &*CBP::ptr() };

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
        let cbp = unsafe { &*CBP::ptr() };

        // Invalidate I-Cache
        cbp.iciallu();

        ::asm::dsb();
        ::asm::isb();
    }

    /// Enables D-cache if currently disabled
    #[inline]
    pub fn enable_dcache(&self, cpuid: &cpuid::RegisterBlock) {
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
    pub fn disable_dcache(&self, cpuid: &cpuid::RegisterBlock) {
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
    fn invalidate_dcache(&self, cpuid: &cpuid::RegisterBlock) {
        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &*CBP::ptr() };

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
    pub fn clean_dcache(&self, cpuid: &cpuid::RegisterBlock) {
        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &*CBP::ptr() };

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
    pub fn clean_invalidate_dcache(&self, cpuid: &cpuid::RegisterBlock) {
        // All of CBP is write-only so no data races are possible
        let cbp = unsafe { &*CBP::ptr() };

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
        let cbp = unsafe { &*CBP::ptr() };

        ::asm::dsb();

        // Cache lines are fixed to 32 bit on Cortex-M7 and not present in earlier Cortex-M
        const LINESIZE: usize = 32;
        let num_lines = ((size - 1) / LINESIZE) + 1;

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
        let cbp = unsafe { &*CBP::ptr() };

        ::asm::dsb();

        // Cache lines are fixed to 32 bit on Cortex-M7 and not present in earlier Cortex-M
        const LINESIZE: usize = 32;
        let num_lines = ((size - 1) / LINESIZE) + 1;

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
        let cbp = unsafe { &*CBP::ptr() };

        ::asm::dsb();

        // Cache lines are fixed to 32 bit on Cortex-M7 and not present in earlier Cortex-M
        const LINESIZE: usize = 32;
        let num_lines = ((size - 1) / LINESIZE) + 1;

        let mut addr = addr & 0xFFFF_FFE0;

        for _ in 0..num_lines {
            cbp.dccimvac(addr as u32);
            addr += LINESIZE;
        }

        ::asm::dsb();
        ::asm::isb();
    }
}
