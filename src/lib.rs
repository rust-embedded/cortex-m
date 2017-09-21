//! Low level access to Cortex-M processors
//!
//! This crate provides:
//!
//! - Access to core peripherals like NVIC, SCB and SysTick.
//! - Access to core registers like CONTROL, MSP and PSR.
//! - Interrupt manipulation mechanisms
//! - Data structures like the vector table
//! - Safe wrappers around assembly instructions like `bkpt`

#![cfg_attr(target_arch = "arm", feature(core_intrinsics))]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(const_unsafe_cell_new)]
#![feature(naked_functions)]
#![no_std]

extern crate aligned;
pub extern crate cortex_m_semihosting as semihosting;
extern crate volatile_register;

#[macro_use]
mod macros;

#[macro_use]
pub mod asm;
pub mod ctxt;
pub mod exception;
pub mod interrupt;
pub mod itm;
pub mod peripheral;
pub mod register;

/// A reserved spot in the vector table
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Reserved {
    /// Reserved
    Vector = 0,
}
