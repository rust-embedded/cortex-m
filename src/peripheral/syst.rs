//! SysTick: System Timer

use volatile_register::{RO, RW};

/// Register block
#[repr(C)]
pub struct RegisterBlock {
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

impl RegisterBlock {
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
            SystClkSource::External => unsafe { self.csr.modify(|v| v & !SYST_CSR_CLKSOURCE) },
            SystClkSource::Core => unsafe { self.csr.modify(|v| v | SYST_CSR_CLKSOURCE) },
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
