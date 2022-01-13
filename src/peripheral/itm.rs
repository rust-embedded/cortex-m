//! Instrumentation Trace Macrocell
//!
//! The documentation in this module contains references to ARM specifications, namely:
//! - coresight: [*ARM CoreSight Architecture Specification*, Version 3.0](https://developer.arm.com/documentation/ihi0029/latest).
//!
//! *NOTE* Not available on Armv6-M and Armv8-M Baseline.

use core::cell::UnsafeCell;
use core::ptr;

use volatile_register::{RO, RW, WO};

use crate::peripheral::ITM;
use bitfield::bitfield;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
    pub lsr: RO<Lsr>,
}

bitfield! {
    /// Trace Control Register.
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Tcr(u32);
    itmena, set_itmena: 0;
    tsena, set_tsena: 1;
    syncena, set_synena: 2;
    txena, set_txena: 3;
    swoena, set_swoena: 4;
    u8, tsprescale, set_tsprescale: 9, 8;
    u8, gtsfreq, set_gtsfreq: 11, 10;
    u8, tracebusid, set_tracebusid: 22, 16;
    busy, _: 23;
}

bitfield! {
    /// Software Lock Status Register
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Lsr(u32);
    sli, _: 0;
    slk, _: 1;
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
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

#[cfg(feature = "std")]
impl core::convert::TryFrom<u8> for LocalTimestampOptions {
    type Error = ();

    /// Converts an integer value to an enabled [LocalTimestampOptions]
    /// variant. Accepted values are: 1, 4, 16, 64. Any other value
    /// yields `Err(())`.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Enabled),
            4 => Ok(Self::EnabledDiv4),
            16 => Ok(Self::EnabledDiv16),
            64 => Ok(Self::EnabledDiv64),
            _ => Err(()),
        }
    }
}

/// The possible global timestamp options.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

/// Available settings for the ITM peripheral.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

/// Possible errors on [ITM::configure].
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ITMConfigurationError {
    /// Global timestamp generation is not supported on this target.
    /// Request [`GlobalTimestampOptions::Disabled`] instead.
    ///
    /// [`ITM_TCR`](struct@Tcr) register remains unchanged on this error.
    GTS,
    /// The requested timestamp clock source is not supported on this target.
    ///
    /// *NOTE*: `GTSFREQ` in [`ITM_TCR`](struct@Tcr) field has
    /// potentially been changed on this error.
    TimestampClkSrc,
    /// The target does not implement the local timestamp prescaler.
    /// Request [`LocalTimestampOptions::Disabled`] or
    /// [`LocalTimestampOptions::Disabled`] instead.
    ///
    /// *NOTE*: `GTSFREQ` and `SWOENA` in [`ITM_TCR`](struct@Tcr) fields
    /// have potentially changed on this error.
    TSPrescale,
}

impl ITM {
    /// Removes the software lock on the [`ITM`]. Must be called before
    /// any mutating [`ITM`] functions if a software lock mechanism is
    /// implemented. See [`has_software_lock`].
    ///
    /// See (coresight, B2.3.10).
    #[inline]
    pub fn unlock(&mut self) {
        // NOTE(unsafe) atomic write to a stateless, write-only register
        unsafe { self.lar.write(0xC5AC_CE55) }
    }

    /// Adds the software lock on the [`ITM`]. Should be called after any other mutating [`ITM`] functions.
    ///
    /// See (coresight, B2.3.10).
    #[inline]
    pub fn lock(&mut self) {
        // NOTE(unsafe) atomic write to a stateless, write-only register
        unsafe { self.lar.write(0) }
    }

    /// Checks whether the target implements the software lock
    /// mechanism. If `true`, [`unlock`] must be called before any other
    /// mutating [`ITM`] functions.
    ///
    /// See (coresight, B2.3.10).
    #[inline]
    pub fn has_software_lock(&self) -> bool {
        self.lsr.read().sli()
    }

    /// Checks whether the peripheral is locked.
    ///
    /// See (coresight, B2.3.10).
    #[inline]
    pub fn locked(&self) -> bool {
        self.lsr.read().slk()
    }

    /// Indicates whether the [`ITM`] is currently processing events.
    /// Returns `true` if [`ITM`] events are present and are being drained.
    #[inline]
    pub fn busy(&self) -> bool {
        self.tcr.read().busy()
    }

    /// Configures the [`ITM`] with the passed [`ITMSettings`]. Returns `true`
    /// if the configuration was successfully applied.
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn configure(&mut self, settings: ITMSettings) -> Result<(), ITMConfigurationError> {
        use ITMConfigurationError as Error;

        // The ITM must be unlocked before we apply any changes.
        if self.has_software_lock() && self.locked() {
            self.unlock();
            while self.locked() {}
        }

        // The ITM must then be disabled before altering certain fields
        // in order to avoid trace stream corruption.
        //
        // NOTE: this is only required before modifying the TraceBusID
        // field, but better be on the safe side for now.
        unsafe {
            self.tcr.modify(|mut r| {
                r.set_itmena(false);
                r
            });
            while self.busy() {}
        }

        unsafe {
            self.tcr.modify(|mut r| {
                r.set_gtsfreq(match settings.global_timestamps {
                    GlobalTimestampOptions::Disabled => 0b00,
                    GlobalTimestampOptions::Every128Cycles => 0b01,
                    GlobalTimestampOptions::Every8192Cycles => 0b10,
                    GlobalTimestampOptions::EveryPacket => 0b11,
                });

                r
            });
        }
        // GTSFREQ is potentially RAZ/WI
        if settings.global_timestamps != GlobalTimestampOptions::Disabled
            && self.tcr.read().gtsfreq() == 0
        {
            return Err(Error::GTS);
        }

        unsafe {
            self.tcr.modify(|mut r| {
                r.set_swoena(match settings.timestamp_clk_src {
                    TimestampClkSrc::SystemClock => false,
                    TimestampClkSrc::AsyncTPIU => true,
                });

                r
            });
        }
        // SWOENA is potentially either RAZ or RAO
        if !{
            match settings.timestamp_clk_src {
                TimestampClkSrc::SystemClock => !self.tcr.read().swoena(),
                TimestampClkSrc::AsyncTPIU => self.tcr.read().swoena(),
            }
        } {
            return Err(Error::TimestampClkSrc);
        }

        unsafe {
            self.tcr.modify(|mut r| {
                r.set_tsprescale(match settings.local_timestamps {
                    LocalTimestampOptions::Disabled | LocalTimestampOptions::Enabled => 0b00,
                    LocalTimestampOptions::EnabledDiv4 => 0b10,
                    LocalTimestampOptions::EnabledDiv16 => 0b10,
                    LocalTimestampOptions::EnabledDiv64 => 0b11,
                });

                r
            })
        }
        // TSPrescale is potentially RAZ/WI
        if settings.local_timestamps != LocalTimestampOptions::Disabled
            && settings.local_timestamps != LocalTimestampOptions::Enabled
            && self.tcr.read().tsprescale() == 0
        {
            return Err(Error::TSPrescale);
        }

        unsafe {
            self.tcr.modify(|mut r| {
                r.set_itmena(settings.enable);
                r.set_tsena(settings.local_timestamps != LocalTimestampOptions::Disabled);
                r.set_txena(settings.forward_dwt); // forward hardware event packets from the DWT to the ITM
                r.set_tracebusid(settings.bus_id.unwrap_or(0));

                r
            });
        }

        if self.has_software_lock() {
            self.lock();
        }

        Ok(())
    }
}
