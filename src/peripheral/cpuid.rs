//! CPUID

use volatile_register::RO;
#[cfg(any(armv7m, test))]
use volatile_register::RW;

#[cfg(armv7m)]
use peripheral::CPUID;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
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
impl CPUID {
    /// Selects the current CCSIDR
    ///
    /// * `level`: the required cache level minus 1, e.g. 0 for L1, 1 for L2
    /// * `ind`: select instruction cache or data/unified cache
    ///
    /// `level` is masked to be between 0 and 7.
    pub fn select_cache(&mut self, level: u8, ind: CsselrCacheType) {
        const CSSELR_IND_POS: u32 = 0;
        const CSSELR_IND_MASK: u32 = 1 << CSSELR_IND_POS;
        const CSSELR_LEVEL_POS: u32 = 1;
        const CSSELR_LEVEL_MASK: u32 = 0x7 << CSSELR_LEVEL_POS;

        unsafe {
            self.csselr.write(
                (((level as u32) << CSSELR_LEVEL_POS) & CSSELR_LEVEL_MASK)
                    | (((ind as u32) << CSSELR_IND_POS) & CSSELR_IND_MASK),
            )
        }
    }

    /// Returns the number of sets and ways in the selected cache
    pub fn cache_num_sets_ways(&mut self, level: u8, ind: CsselrCacheType) -> (u16, u16) {
        const CCSIDR_NUMSETS_POS: u32 = 13;
        const CCSIDR_NUMSETS_MASK: u32 = 0x7FFF << CCSIDR_NUMSETS_POS;
        const CCSIDR_ASSOCIATIVITY_POS: u32 = 3;
        const CCSIDR_ASSOCIATIVITY_MASK: u32 = 0x3FF << CCSIDR_ASSOCIATIVITY_POS;

        self.select_cache(level, ind);
        ::asm::dsb();
        let ccsidr = self.ccsidr.read();
        (
            (1 + ((ccsidr & CCSIDR_NUMSETS_MASK) >> CCSIDR_NUMSETS_POS)) as u16,
            (1 + ((ccsidr & CCSIDR_ASSOCIATIVITY_MASK) >> CCSIDR_ASSOCIATIVITY_POS)) as u16,
        )
    }
}
