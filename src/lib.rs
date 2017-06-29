//! Low level access to Cortex-M processors
//!
//! This crate provides:
//!
//! - Access to core peripherals like NVIC, SCB and SysTick.
//! - Access to core registers like CONTROL, MSP and PSR.
//! - Interrupt manipulation mechanisms
//! - Data structures like the vector table
//! - Safe wrappers around assembly instructions like `bkpt`

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]

extern crate aligned;
pub extern crate cortex_m_semihosting as semihosting;
extern crate volatile_register;

#[macro_use]
mod macros;

#[macro_use]
pub mod asm;
pub mod exception;
pub mod interrupt;
pub mod itm;
pub mod peripheral;
pub mod register;
