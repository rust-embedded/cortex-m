//! Cache and branch predictor maintenance operations

use volatile_register::WO;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
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

const CBP_SW_WAY_POS: u32 = 30;
const CBP_SW_WAY_MASK: u32 = 0x3 << CBP_SW_WAY_POS;
const CBP_SW_SET_POS: u32 = 5;
const CBP_SW_SET_MASK: u32 = 0x1FF << CBP_SW_SET_POS;

impl RegisterBlock {
    /// I-cache invalidate all to PoU
    #[inline(always)]
    pub fn iciallu(&self) {
        unsafe {
            self.iciallu.write(0);
        }
    }

    /// I-cache invalidate by MVA to PoU
    #[inline(always)]
    pub fn icimvau(&self, mva: u32) {
        unsafe {
            self.icimvau.write(mva);
        }
    }

    /// D-cache invalidate by MVA to PoC
    #[inline(always)]
    pub fn dcimvac(&self, mva: u32) {
        unsafe {
            self.dcimvac.write(mva);
        }
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
        unsafe {
            self.dcisw.write(
                (((way as u32) & (CBP_SW_WAY_MASK >> CBP_SW_WAY_POS)) << CBP_SW_WAY_POS)
                    | (((set as u32) & (CBP_SW_SET_MASK >> CBP_SW_SET_POS)) << CBP_SW_SET_POS),
            );
        }
    }

    /// D-cache clean by MVA to PoU
    #[inline(always)]
    pub fn dccmvau(&self, mva: u32) {
        unsafe {
            self.dccmvau.write(mva);
        }
    }

    /// D-cache clean by MVA to PoC
    #[inline(always)]
    pub fn dccmvac(&self, mva: u32) {
        unsafe {
            self.dccmvac.write(mva);
        }
    }

    /// D-cache clean by set-way
    ///
    /// `set` is masked to be between 0 and 3, and `way` between 0 and 511.
    #[inline(always)]
    pub fn dccsw(&self, set: u16, way: u16) {
        // See comment for dcisw() about the format here
        unsafe {
            self.dccsw.write(
                (((way as u32) & (CBP_SW_WAY_MASK >> CBP_SW_WAY_POS)) << CBP_SW_WAY_POS)
                    | (((set as u32) & (CBP_SW_SET_MASK >> CBP_SW_SET_POS)) << CBP_SW_SET_POS),
            );
        }
    }

    /// D-cache clean and invalidate by MVA to PoC
    #[inline(always)]
    pub fn dccimvac(&self, mva: u32) {
        unsafe {
            self.dccimvac.write(mva);
        }
    }

    /// D-cache clean and invalidate by set-way
    ///
    /// `set` is masked to be between 0 and 3, and `way` between 0 and 511.
    #[inline(always)]
    pub fn dccisw(&self, set: u16, way: u16) {
        // See comment for dcisw() about the format here
        unsafe {
            self.dccisw.write(
                (((way as u32) & (CBP_SW_WAY_MASK >> CBP_SW_WAY_POS)) << CBP_SW_WAY_POS)
                    | (((set as u32) & (CBP_SW_SET_MASK >> CBP_SW_SET_POS)) << CBP_SW_SET_POS),
            );
        }
    }

    /// Branch predictor invalidate all
    #[inline(always)]
    pub fn bpiall(&self) {
        unsafe {
            self.bpiall.write(0);
        }
    }
}
