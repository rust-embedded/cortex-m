//! Data Watchpoint and Trace unit

#[cfg(not(armv6m))]
use volatile_register::WO;
use volatile_register::{RO, RW};

use crate::peripheral::DWT;
use bitfield::bitfield;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Control
    pub ctrl: RW<Ctrl>,
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

bitfield! {
    /// Control register.
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Ctrl(u32);
    cyccntena, set_cyccntena: 0;
    pcsamplena, set_pcsamplena: 12;
    exctrcena, set_exctrcena: 16;
    noprfcnt, _: 24;
    nocyccnt, _: 25;
    noexttrig, _: 26;
    notrcpkt, _: 27;
    u8, numcomp, _: 31, 28;
}

/// Comparator
#[repr(C)]
pub struct Comparator {
    /// Comparator
    pub comp: RW<u32>,
    /// Comparator Mask
    pub mask: RW<u32>,
    /// Comparator Function
    pub function: RW<Function>,
    reserved: u32,
}

bitfield! {
    #[repr(C)]
    #[derive(Copy, Clone)]
    /// Comparator FUNCTIONn register.
    pub struct Function(u32);
    u8, function, set_function: 3, 0;
    emitrange, set_emitrange: 5;
    cycmatch, set_cycmatch: 7;
    datavmatch, set_datavmatch: 8;
    matched, _: 24;
}

impl DWT {
    /// Number of comparators implemented
    ///
    /// A value of zero indicates no comparator support.
    #[inline]
    pub fn num_comp(&self) -> u8 {
        self.ctrl.read().numcomp()
    }

    /// Returns `true` if the the implementation supports sampling and exception tracing
    #[cfg(not(armv6m))]
    #[inline]
    pub fn has_exception_trace(&self) -> bool {
        !self.ctrl.read().notrcpkt()
    }

    /// Returns `true` if the implementation includes external match signals
    #[cfg(not(armv6m))]
    #[inline]
    pub fn has_external_match(&self) -> bool {
        !self.ctrl.read().noexttrig()
    }

    /// Returns `true` if the implementation supports a cycle counter
    #[cfg(not(armv6m))]
    #[inline]
    pub fn has_cycle_counter(&self) -> bool {
        !self.ctrl.read().nocyccnt()
    }

    /// Returns `true` if the implementation the profiling counters
    #[cfg(not(armv6m))]
    #[inline]
    pub fn has_profiling_counter(&self) -> bool {
        !self.ctrl.read().noprfcnt()
    }

    /// Enables the cycle counter
    ///
    /// The global trace enable ([`DCB::enable_trace`]) should be set before
    /// enabling the cycle counter, the processor may ignore writes to the
    /// cycle counter enable if the global trace is disabled
    /// (implementation defined behaviour).
    ///
    /// [`DCB::enable_trace`]: crate::peripheral::DCB::enable_trace
    #[cfg(not(armv6m))]
    #[inline]
    pub fn enable_cycle_counter(&mut self) {
        unsafe {
            self.ctrl.modify(|mut r| {
                r.set_cyccntena(true);
                r
            });
        }
    }

    /// Returns `true` if the cycle counter is enabled
    #[cfg(not(armv6m))]
    #[inline]
    pub fn cycle_counter_enabled(&self) -> bool {
        self.ctrl.read().cyccntena()
    }

    /// Whether to enable exception tracing
    #[cfg(not(armv6m))]
    #[inline]
    pub fn enable_exception_tracing(&mut self, bit: bool) {
        unsafe {
            self.ctrl.modify(|mut r| {
                r.set_exctrcena(bit);
                r
            });
        }
    }

    /// Whether to periodically generate PC samples
    #[cfg(not(armv6m))]
    #[inline]
    pub fn enable_pc_samples(&mut self, bit: bool) {
        unsafe {
            self.ctrl.modify(|mut r| {
                r.set_pcsamplena(bit);
                r
            });
        }
    }

    /// Returns the current clock cycle count
    #[cfg(not(armv6m))]
    #[inline]
    #[deprecated(
        since = "0.7.4",
        note = "Use `cycle_count` which follows the C-GETTER convention"
    )]
    pub fn get_cycle_count() -> u32 {
        Self::cycle_count()
    }

    /// Returns the current clock cycle count
    #[cfg(not(armv6m))]
    #[inline]
    pub fn cycle_count() -> u32 {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::ptr()).cyccnt.read() }
    }

    /// Set the cycle count
    #[cfg(not(armv6m))]
    #[inline]
    pub fn set_cycle_count(&mut self, count: u32) {
        unsafe { self.cyccnt.write(count) }
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

    /// Get the CPI count
    ///
    /// Counts additional cycles required to execute multi-cycle instructions,
    /// except those recorded by [`lsu_count`], and counts any instruction fetch
    /// stalls.
    ///
    /// [`lsu_count`]: DWT::lsu_count
    #[cfg(not(armv6m))]
    #[inline]
    pub fn cpi_count() -> u8 {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::ptr()).cpicnt.read() as u8 }
    }

    /// Set the CPI count
    #[cfg(not(armv6m))]
    #[inline]
    pub fn set_cpi_count(&mut self, count: u8) {
        unsafe { self.cpicnt.write(count as u32) }
    }

    /// Get the total cycles spent in exception processing
    #[cfg(not(armv6m))]
    #[inline]
    pub fn exception_count() -> u8 {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::ptr()).exccnt.read() as u8 }
    }

    /// Set the exception count
    #[cfg(not(armv6m))]
    #[inline]
    pub fn set_exception_count(&mut self, count: u8) {
        unsafe { self.exccnt.write(count as u32) }
    }

    /// Get the total number of cycles that the processor is sleeping
    ///
    /// ARM recommends that this counter counts all cycles when the processor is sleeping,
    /// regardless of whether a WFI or WFE instruction, or the sleep-on-exit functionality,
    /// caused the entry to sleep mode.
    /// However, all sleep features are implementation defined and therefore when
    /// this counter counts is implementation defined.
    #[cfg(not(armv6m))]
    #[inline]
    pub fn sleep_count() -> u8 {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::ptr()).sleepcnt.read() as u8 }
    }

    /// Set the sleep count
    #[cfg(not(armv6m))]
    #[inline]
    pub fn set_sleep_count(&mut self, count: u8) {
        unsafe { self.sleepcnt.write(count as u32) }
    }

    /// Get the additional cycles required to execute all load or store instructions
    #[cfg(not(armv6m))]
    #[inline]
    pub fn lsu_count() -> u8 {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::ptr()).lsucnt.read() as u8 }
    }

    /// Set the lsu count
    #[cfg(not(armv6m))]
    #[inline]
    pub fn set_lsu_count(&mut self, count: u8) {
        unsafe { self.lsucnt.write(count as u32) }
    }

    /// Get the folded instruction count
    ///
    /// Increments on each instruction that takes 0 cycles.
    #[cfg(not(armv6m))]
    #[inline]
    pub fn fold_count() -> u8 {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::ptr()).foldcnt.read() as u8 }
    }

    /// Set the folded instruction count
    #[cfg(not(armv6m))]
    #[inline]
    pub fn set_fold_count(&mut self, count: u8) {
        unsafe { self.foldcnt.write(count as u32) }
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
    ///
    /// *NOTE* only compatible with [AccessType::ReadWrite].
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
    #[inline]
    pub fn configure(&self, settings: ComparatorFunction) -> Result<(), DWTError> {
        match settings {
            ComparatorFunction::Address(settings) => unsafe {
                if settings.emit == EmitOption::PC && settings.access_type != AccessType::ReadWrite
                {
                    return Err(DWTError::InvalidFunction);
                }

                self.function.modify(|mut r| {
                    // don't compare data value
                    r.set_datavmatch(false);

                    // don't compare cycle counter value
                    // NOTE: only needed for comparator 0, but is SBZP.
                    r.set_cycmatch(false);

                    // FUNCTION, EMITRANGE
                    // See Table C1-14
                    let (function, emit_range) = match (&settings.access_type, &settings.emit) {
                        (AccessType::ReadOnly, EmitOption::Data) => (0b1100, false),
                        (AccessType::ReadOnly, EmitOption::Address) => (0b1100, true),
                        (AccessType::ReadOnly, EmitOption::AddressData) => (0b1110, true),
                        (AccessType::ReadOnly, EmitOption::PCData) => (0b1110, false),

                        (AccessType::WriteOnly, EmitOption::Data) => (0b1101, false),
                        (AccessType::WriteOnly, EmitOption::Address) => (0b1101, true),
                        (AccessType::WriteOnly, EmitOption::AddressData) => (0b1111, true),
                        (AccessType::WriteOnly, EmitOption::PCData) => (0b1111, false),

                        (AccessType::ReadWrite, EmitOption::Data) => (0b0010, false),
                        (AccessType::ReadWrite, EmitOption::Address) => (0b0001, true),
                        (AccessType::ReadWrite, EmitOption::AddressData) => (0b0010, true),
                        (AccessType::ReadWrite, EmitOption::PCData) => (0b0011, false),

                        (AccessType::ReadWrite, EmitOption::PC) => (0b0001, false),
                        (_, EmitOption::PC) => unreachable!(), // cannot return Err here; handled above
                    };
                    r.set_function(function);
                    r.set_emitrange(emit_range);

                    r
                });

                self.comp.write(settings.address);
                self.mask.write(settings.mask);
            },
        }

        Ok(())
    }
}
