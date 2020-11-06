//! Security Attribution Unit
//!
//! *NOTE* Available only on Armv8-M and Armv8.1-M, for the following Rust target triples:
//!   * `thumbv8m.base-none-eabi`
//!   * `thumbv8m.main-none-eabi`
//!   * `thumbv8m.main-none-eabihf`
//!
//! For reference please check the section B8.3 of the Armv8-M Architecture Reference Manual.

use crate::interrupt;
use crate::peripheral::SAU;
use modular_bitfield::prelude::*;
use volatile_register::{RO, RW};

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Control Register
    pub ctrl: RW<Ctrl>,
    /// Type Register
    pub _type: RO<Type>,
    /// Region Number Register
    pub rnr: RW<Rnr>,
    /// Region Base Address Register
    pub rbar: RW<Rbar>,
    /// Region Limit Address Register
    pub rlar: RW<Rlar>,
    /// Secure Fault Status Register
    pub sfsr: RO<Sfsr>,
    /// Secure Fault Address Register
    pub sfar: RO<Sfar>,
}

#[bitfield(bits = 32)]
#[derive(Copy, Clone)]
#[repr(u32)]
/// Control Register description.
pub struct Ctrl {
    pub enable: bool,
    pub allns: bool,
    #[skip]
    __: B30,
}

#[bitfield(bits = 32)]
#[derive(Copy, Clone)]
#[repr(C, u32)]
/// Type Register description.
pub struct Type {
    #[skip(setters)]
    pub sregion: u8,
    #[skip]
    __: B24,
}

#[bitfield(bits = 32)]
#[derive(Copy, Clone)]
#[repr(C, u32)]
/// Region Number Register description.
pub struct Rnr {
    pub region: u8,
    #[skip]
    __: B24,
}

#[bitfield(bits = 32)]
#[derive(Copy, Clone)]
#[repr(C, u32)]
/// Region Base Address Register description.
pub struct Rbar {
    #[skip]
    __: B5,
    pub baddr: B27,
}

#[bitfield(bits = 32)]
#[derive(Copy, Clone)]
#[repr(C, u32)]
/// Region Limit Address Register description.
pub struct Rlar {
    pub enable: bool,
    pub nsc: bool,
    #[skip]
    __: B3,
    pub laddr: B27,
}

#[bitfield(bits = 32)]
#[derive(Copy, Clone)]
#[repr(C, u32)]
/// Secure Fault Status Register description.
pub struct Sfsr {
    #[skip(setters)]
    pub invep: bool,
    #[skip(setters)]
    pub invis: bool,
    #[skip(setters)]
    pub inver: bool,
    #[skip(setters)]
    pub auviol: bool,
    #[skip(setters)]
    pub invtran: bool,
    #[skip(setters)]
    pub lsperr: bool,
    #[skip(setters)]
    pub sfarvalid: bool,
    #[skip(setters)]
    pub lserr: bool,
    #[skip]
    __: B24,
}

#[bitfield(bits = 32)]
#[derive(Copy, Clone)]
#[repr(C, u32)]
/// Secure Fault Address Register description.
pub struct Sfar {
    #[skip(setters)]
    pub address: u32,
}

/// Possible attribute of a SAU region.
#[derive(Debug)]
pub enum SauRegionAttribute {
    /// SAU region is Secure
    Secure,
    /// SAU region is Non-Secure Callable
    NonSecureCallable,
    /// SAU region is Non-Secure
    NonSecure,
}

/// Description of a SAU region.
#[derive(Debug)]
pub struct SauRegion {
    /// First address of the region, its 5 least significant bits must be set to zero.
    pub base_address: u32,
    /// Last address of the region, its 5 least significant bits must be set to one.
    pub limit_address: u32,
    /// Attribute of the region.
    pub attribute: SauRegionAttribute,
}

/// Possible error values returned by the SAU methods.
#[derive(Debug)]
pub enum SauError {
    /// The region number parameter to set or get a region must be between 0 and
    /// region_numbers() - 1.
    RegionNumberTooBig,
    /// Bits 0 to 4 of the base address of a SAU region must be set to zero.
    WrongBaseAddress,
    /// Bits 0 to 4 of the limit address of a SAU region must be set to one.
    WrongLimitAddress,
}

impl SAU {
    /// Get the number of implemented SAU regions.
    #[inline]
    pub fn region_numbers(&self) -> u8 {
        self._type.read().sregion()
    }

    /// Enable the SAU.
    #[inline]
    pub fn enable(&mut self) {
        unsafe {
            self.ctrl.modify(|mut ctrl| {
                ctrl.set_enable(true);
                ctrl
            });
        }
    }

    /// Set a SAU region to a region number.
    /// SAU regions must be 32 bytes aligned and their sizes must be a multiple of 32 bytes. It
    /// means that the 5 least significant bits of the base address of a SAU region must be set to
    /// zero and the 5 least significant bits of the limit address must be set to one.
    /// The region number must be valid.
    /// This function is executed under a critical section to prevent having inconsistent results.
    #[inline]
    pub fn set_region(&mut self, region_number: u8, region: SauRegion) -> Result<(), SauError> {
        interrupt::free(|_| {
            let base_address = region.base_address;
            let limit_address = region.limit_address;
            let attribute = region.attribute;

            if region_number >= self.region_numbers() {
                Err(SauError::RegionNumberTooBig)
            } else if base_address & 0x1F != 0 {
                Err(SauError::WrongBaseAddress)
            } else if limit_address & 0x1F != 0x1F {
                Err(SauError::WrongLimitAddress)
            } else {
                // All fields of these registers are going to be modified so we don't need to read them
                // before.
                let mut rnr = Rnr::from(0u32);
                let mut rbar = Rbar::from(0u32);
                let mut rlar = Rlar::from(0u32);

                rnr.set_region(region_number);
                rbar.set_baddr(base_address >> 5);
                rlar.set_laddr(limit_address >> 5);

                match attribute {
                    SauRegionAttribute::Secure => {
                        rlar.set_nsc(false);
                        rlar.set_enable(false);
                    }
                    SauRegionAttribute::NonSecureCallable => {
                        rlar.set_nsc(true);
                        rlar.set_enable(true);
                    }
                    SauRegionAttribute::NonSecure => {
                        rlar.set_nsc(false);
                        rlar.set_enable(true);
                    }
                }

                unsafe {
                    self.rnr.write(rnr);
                    self.rbar.write(rbar);
                    self.rlar.write(rlar);
                }

                Ok(())
            }
        })
    }

    /// Get a region from the SAU.
    /// The region number must be valid.
    /// This function is executed under a critical section to prevent having inconsistent results.
    #[inline]
    pub fn get_region(&mut self, region_number: u8) -> Result<SauRegion, SauError> {
        interrupt::free(|_| {
            if region_number >= self.region_numbers() {
                Err(SauError::RegionNumberTooBig)
            } else {
                unsafe {
                    self.rnr.write(Rnr::from(region_number as u32));
                }

                let rbar = self.rbar.read();
                let rlar = self.rlar.read();

                let attribute = match (rlar.enable(), rlar.nsc()) {
                    (false, _) => SauRegionAttribute::Secure,
                    (true, false) => SauRegionAttribute::NonSecure,
                    (true, true) => SauRegionAttribute::NonSecureCallable,
                };

                Ok(SauRegion {
                    base_address: rbar.baddr() << 5,
                    limit_address: (rlar.laddr() << 5) | 0x1F,
                    attribute,
                })
            }
        })
    }
}
