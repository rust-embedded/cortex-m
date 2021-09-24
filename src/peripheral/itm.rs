//! Instrumentation Trace Macrocell
//!
//! *NOTE* Not available on Armv6-M and Armv8-M Baseline.

use core::cell::UnsafeCell;
use core::ptr;

use volatile_register::{RO, RW, WO};

use crate::peripheral::ITM;
use bitfield::bitfield;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
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
    pub tcr: RW<Tcr>,
    reserved3: [u32; 75],
    /// Lock Access
    pub lar: WO<u32>,
    /// Lock Status
    pub lsr: RO<u32>,
}

bitfield! {
    /// Trace Control Register.
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Tcr(u32);
    get_itmena, set_itmena: 0;
    get_tsena, set_tsena: 1;
    get_syncena, set_synena: 2;
    get_txena, set_txena: 3;
    get_swoena, set_swoena: 4;
    u8, get_tsprescale, set_tsprescale: 9, 8;
    u8, get_gtsfreq, set_gtsfreq: 11, 10;
    u8, get_tracebusid, set_tracebusid: 22, 16;
    busy, _: 23;
}

/// Stimulus Port
pub struct Stim {
    register: UnsafeCell<u32>,
}

impl Stim {
    /// Writes an `u8` payload into the stimulus port
    #[inline]
    pub fn write_u8(&mut self, value: u8) {
        unsafe { ptr::write_volatile(self.register.get() as *mut u8, value) }
    }

    /// Writes an `u16` payload into the stimulus port
    #[inline]
    pub fn write_u16(&mut self, value: u16) {
        unsafe { ptr::write_volatile(self.register.get() as *mut u16, value) }
    }

    /// Writes an `u32` payload into the stimulus port
    #[inline]
    pub fn write_u32(&mut self, value: u32) {
        unsafe { ptr::write_volatile(self.register.get(), value) }
    }

    /// Returns `true` if the stimulus port is ready to accept more data
    #[cfg(not(armv8m))]
    #[inline]
    pub fn is_fifo_ready(&self) -> bool {
        unsafe { ptr::read_volatile(self.register.get()) & 0b1 == 1 }
    }

    /// Returns `true` if the stimulus port is ready to accept more data
    #[cfg(armv8m)]
    #[inline]
    pub fn is_fifo_ready(&self) -> bool {
        // ARMv8-M adds a disabled bit; we indicate that we are ready to
        // proceed with a stimulus write if the port is either ready (bit 0) or
        // disabled (bit 1).
        unsafe { ptr::read_volatile(self.register.get()) & 0b11 != 0 }
    }
}

/// The possible local timestamp options.
#[derive(Debug, PartialEq)]
pub enum LocalTimestampOptions {
    /// Disable local timestamps.
    Disabled,
    /// Enable local timestamps and use no prescaling.
    Enabled,
    /// Enable local timestamps and set the prescaler to divide the
    /// reference clock by 4.
    EnabledDiv4,
    /// Enable local timestamps and set the prescaler to divide the
    /// reference clock by 16.
    EnabledDiv16,
    /// Enable local timestamps and set the prescaler to divide the
    /// reference clock by 64.
    EnabledDiv64,
}

/// The possible global timestamp options.
#[derive(Debug)]
pub enum GlobalTimestampOptions {
    /// Disable global timestamps.
    Disabled,
    /// Generate a global timestamp approximately every 128 cycles.
    Every128Cycles,
    /// Generate a global timestamp approximately every 8921 cycles.
    Every8192Cycles,
    /// Generate a global timestamp after every packet, if the output FIFO is empty.
    EveryPacket,
}

/// The possible clock sources for timestamp counters.
#[derive(Debug)]
pub enum TimestampClkSrc {
    /// Clock timestamp counters using the system processor clock.
    SystemClock,
    /// Clock timestamp counters using the asynchronous clock from the
    /// TPIU interface.
    ///
    /// NOTE: The timestamp counter is held in reset while the output
    /// line is idle.
    AsyncTPIU,
}

/// blah
#[derive(Debug)]
pub struct ITMSettings {
    /// Whether to enable ITM.
    pub enable: bool,
    /// Whether DWT packets should be forwarded to ITM.
    pub forward_dwt: bool,
    /// The local timestamp options that should be applied.
    pub local_timestamps: LocalTimestampOptions,
    /// The global timestamp options that should be applied.
    pub global_timestamps: GlobalTimestampOptions,
    /// The trace bus ID to use when multi-trace sources are in use.
    /// `None` specifies that only a single trace source is in use and
    /// has the same effect as `Some(0)`.
    pub bus_id: Option<u8>,
    /// The clock that should increase timestamp counters.
    pub timestamp_clk_src: TimestampClkSrc,
}

impl ITM {
    /// Removes the software lock on the ITM.
    #[inline]
    pub fn unlock(&mut self) {
        // NOTE(unsafe) atomic write to a stateless, write-only register
        unsafe { self.lar.write(0xC5AC_CE55) }
    }

    /// Configures the ITM with the passed [ITMSettings].
    #[inline]
    pub fn configure(&mut self, settings: ITMSettings) {
        unsafe {
            self.tcr.modify(|mut r| {
                r.set_itmena(settings.enable);
                r.set_tsena(settings.local_timestamps != LocalTimestampOptions::Disabled);
                r.set_txena(settings.forward_dwt);
                r.set_tsprescale(match settings.local_timestamps {
                    LocalTimestampOptions::Disabled | LocalTimestampOptions::Enabled => 0b00,
                    LocalTimestampOptions::EnabledDiv4 => 0b10,
                    LocalTimestampOptions::EnabledDiv16 => 0b10,
                    LocalTimestampOptions::EnabledDiv64 => 0b11,
                });
                r.set_gtsfreq(match settings.global_timestamps {
                    GlobalTimestampOptions::Disabled => 0b00,
                    GlobalTimestampOptions::Every128Cycles => 0b01,
                    GlobalTimestampOptions::Every8192Cycles => 0b10,
                    GlobalTimestampOptions::EveryPacket => 0b11,
                });
                r.set_swoena(match settings.timestamp_clk_src {
                    TimestampClkSrc::SystemClock => false,
                    TimestampClkSrc::AsyncTPIU => true,
                });
                r.set_tracebusid(settings.bus_id.unwrap_or(0));

                r
            });
        }
    }
}
