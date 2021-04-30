//! Data Watchpoint and Trace unit

#[cfg(not(armv6m))]
use volatile_register::WO;
use volatile_register::{RO, RW};

use crate::peripheral::DWT;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Control
    pub ctrl: RW<u32>,
    /// Cycle Count
    #[cfg(not(armv6m))]
    pub cyccnt: RW<u32>,
    /// CPI Count
    #[cfg(not(armv6m))]
    pub cpicnt: RW<u32>,
    /// Exception Overhead Count
    #[cfg(not(armv6m))]
    pub exccnt: RW<u32>,
    /// Sleep Count
    #[cfg(not(armv6m))]
    pub sleepcnt: RW<u32>,
    /// LSU Count
    #[cfg(not(armv6m))]
    pub lsucnt: RW<u32>,
    /// Folded-instruction Count
    #[cfg(not(armv6m))]
    pub foldcnt: RW<u32>,
    /// Cortex-M0(+) does not have  these parts
    #[cfg(armv6m)]
    reserved: [u32; 6],
    /// Program Counter Sample
    pub pcsr: RO<u32>,
    /// Comparators
    #[cfg(armv6m)]
    pub c: [Comparator; 2],
    #[cfg(not(armv6m))]
    /// Comparators
    pub c: [Comparator; 16],
    #[cfg(not(armv6m))]
    reserved: [u32; 932],
    /// Lock Access
    #[cfg(not(armv6m))]
    pub lar: WO<u32>,
    /// Lock Status
    #[cfg(not(armv6m))]
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

impl DWT {
    /// Enables the cycle counter
    #[cfg(not(armv6m))]
    #[inline]
    pub fn enable_cycle_counter(&mut self) {
        unsafe { self.ctrl.modify(|r| r | 1) }
    }

    /// Whether to enable exception tracing
    // TODO find out if this is supported om armv6m
    #[inline]
    pub fn enable_exception_tracing(&mut self, bit: bool) {
        unsafe {
            // EXCTRCENA
            if bit {
                self.ctrl.modify(|r| r | (1 << 16));
            } else {
                self.ctrl.modify(|r| r & !(1 << 16));
            }
        }
    }

    /// Whether to periodically generate PC samples
    // TODO find out if this is supported on armv6m
    #[inline]
    pub fn enable_pc_samples(&mut self, bit: bool) {
        unsafe {
            // PCSAMPLENA
            if bit {
                self.ctrl.modify(|r| r | (1 << 12));
            } else {
                self.ctrl.modify(|r| r & !(1 << 12));
            }
        }
    }

    /// Returns the current clock cycle count
    #[cfg(not(armv6m))]
    #[inline]
    pub fn get_cycle_count() -> u32 {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::ptr()).cyccnt.read() }
    }

    /// Removes the software lock on the DWT
    ///
    /// Some devices, like the STM32F7, software lock the DWT after a power cycle.
    #[cfg(not(armv6m))]
    #[inline]
    pub fn unlock() {
        // NOTE(unsafe) atomic write to a stateless, write-only register
        unsafe { (*Self::ptr()).lar.write(0xC5AC_CE55) }
    }
}

/// Whether the comparator should match on read, write or read/write operations.
#[derive(Debug, PartialEq)]
pub enum AccessType {
    /// Generate packet only when matched adress is read from.
    ReadOnly,
    /// Generate packet only when matched adress is written to.
    WriteOnly,
    /// Generate packet when matched adress is both read from and written to.
    ReadWrite,
}

/// The sequence of packet(s) that should be emitted on comparator match.
#[derive(Debug, PartialEq)]
pub enum EmitOption {
    /// Emit only trace data value packet.
    Data,
    /// Emit only trace address packet.
    Address,
    /// Emit only trace PC value packet
    /// NOTE: only compatible with [AccessType::ReadWrite].
    PC,
    /// Emit trace address and data value packets.
    AddressData,
    /// Emit trace PC value and data value packets.
    PCData,
}

/// Settings for address matching
#[derive(Debug)]
pub struct ComparatorAddressSettings {
    /// The address to match against.
    pub address: u32,
    /// The address mask to match against.
    pub mask: u32,
    /// What sequence of packet(s) to emit on comparator match.
    pub emit: EmitOption,
    /// Whether to match on read, write or read/write operations.
    pub access_type: AccessType,
}

/// The available functions of a DWT comparator.
#[derive(Debug)]
pub enum ComparatorFunction {
    /// Compare accessed memory addresses.
    Address(ComparatorAddressSettings),
}

/// Possible error values returned on [Comparator::configure].
#[derive(Debug)]
pub enum DWTError {
    /// Invalid combination of [AccessType] and [EmitOption].
    InvalidFunction,
}

impl Comparator {
    /// Configure the function of the comparator
    pub fn configure(&mut self, settings: ComparatorFunction) -> Result<(), DWTError> {
        match settings {
            ComparatorFunction::Address(settings) => unsafe {
                if settings.emit == EmitOption::PC && settings.access_type != AccessType::ReadWrite
                {
                    return Err(DWTError::InvalidFunction);
                }

                self.function.modify(|mut r| {
                    // clear DATAVMATCH; dont compare data value
                    r &= !(1 << 8);

                    // clear CYCMATCH: dont compare cycle counter value
                    // NOTE: only needed for comparator 0, but is SBZP
                    r &= !(1 << 7);

                    let mut set_function = |fun, emit_range| {
                        r &= u32::MAX << 4; // zero the FUNCTION field first
                        r |= fun;

                        if emit_range {
                            r |= 1 << 5;
                        } else {
                            r &= !(1 << 5);
                        }
                    };

                    // FUNCTION, EMITRANGE
                    // See Table C1-14
                    match (&settings.access_type, &settings.emit) {
                        (AccessType::ReadOnly, EmitOption::Data) => {
                            set_function(0b1100, false);
                        }
                        (AccessType::ReadOnly, EmitOption::Address) => {
                            set_function(0b1100, true);
                        }
                        (AccessType::ReadOnly, EmitOption::AddressData) => {
                            set_function(0b1110, true);
                        }
                        (AccessType::ReadOnly, EmitOption::PCData) => {
                            set_function(0b1110, false);
                        }

                        (AccessType::WriteOnly, EmitOption::Data) => {
                            set_function(0b1101, false);
                        }
                        (AccessType::WriteOnly, EmitOption::Address) => {
                            set_function(0b1101, true);
                        }
                        (AccessType::WriteOnly, EmitOption::AddressData) => {
                            set_function(0b1111, true);
                        }
                        (AccessType::WriteOnly, EmitOption::PCData) => {
                            set_function(0b1111, false);
                        }

                        (AccessType::ReadWrite, EmitOption::Data) => {
                            set_function(0b0010, false);
                        }
                        (AccessType::ReadWrite, EmitOption::Address) => {
                            set_function(0b0001, true);
                        }
                        (AccessType::ReadWrite, EmitOption::AddressData) => {
                            set_function(0b0010, true);
                        }
                        (AccessType::ReadWrite, EmitOption::PCData) => {
                            set_function(0b0011, false);
                        }

                        (AccessType::ReadWrite, EmitOption::PC) => {
                            set_function(0b0001, false);
                        }
                        (_, EmitOption::PC) => unreachable!(), // cannot return Err here; handled above
                    }

                    r
                });

                self.comp.write(settings.address);
                self.mask.write(settings.mask);
            },
        }

        Ok(())
    }
}
