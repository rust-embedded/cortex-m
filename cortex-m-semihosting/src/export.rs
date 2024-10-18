//! IMPLEMENTATION DETAILS USED BY MACROS

// This must be replaced by a different solution before rust edition 2024
// https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html
#![allow(static_mut_refs)]

use core::fmt::{self, Write};

use crate::hio::{self, HostStream};

static mut HSTDOUT: Option<HostStream> = None;

pub fn hstdout_str(s: &str) {
    let _result = critical_section::with(|_| unsafe {
        if HSTDOUT.is_none() {
            HSTDOUT = Some(hio::hstdout()?);
        }

        HSTDOUT.as_mut().unwrap().write_str(s).map_err(drop)
    });
}

pub fn hstdout_fmt(args: fmt::Arguments) {
    let _result = critical_section::with(|_| unsafe {
        if HSTDOUT.is_none() {
            HSTDOUT = Some(hio::hstdout()?);
        }

        HSTDOUT.as_mut().unwrap().write_fmt(args).map_err(drop)
    });
}

static mut HSTDERR: Option<HostStream> = None;

pub fn hstderr_str(s: &str) {
    let _result = critical_section::with(|_| unsafe {
        if HSTDERR.is_none() {
            HSTDERR = Some(hio::hstderr()?);
        }

        HSTDERR.as_mut().unwrap().write_str(s).map_err(drop)
    });
}

pub fn hstderr_fmt(args: fmt::Arguments) {
    let _result = critical_section::with(|_| unsafe {
        if HSTDERR.is_none() {
            HSTDERR = Some(hio::hstderr()?);
        }

        HSTDERR.as_mut().unwrap().write_fmt(args).map_err(drop)
    });
}
