//! IMPLEMENTATION DETAILS USED BY MACROS

use core::fmt::{self, Write};

use cortex_m::interrupt;

use crate::hio::{self, HStderr, HStdout};

static mut HSTDOUT: Option<HStdout> = None;

pub fn hstdout_str(s: &str) {
    let _result = interrupt::free(|_| unsafe {
        if HSTDOUT.is_none() {
            HSTDOUT = Some(hio::hstdout()?);
        }

        HSTDOUT.as_mut().unwrap().write_str(s).map_err(drop)
    });
}

pub fn hstdout_fmt(args: fmt::Arguments) {
    let _result = interrupt::free(|_| unsafe {
        if HSTDOUT.is_none() {
            HSTDOUT = Some(hio::hstdout()?);
        }

        HSTDOUT.as_mut().unwrap().write_fmt(args).map_err(drop)
    });
}

static mut HSTDERR: Option<HStderr> = None;

pub fn hstderr_str(s: &str) {
    let _result = interrupt::free(|_| unsafe {
        if HSTDERR.is_none() {
            HSTDERR = Some(hio::hstderr()?);
        }

        HSTDERR.as_mut().unwrap().write_str(s).map_err(drop)
    });
}

pub fn hstderr_fmt(args: fmt::Arguments) {
    let _result = interrupt::free(|_| unsafe {
        if HSTDERR.is_none() {
            HSTDERR = Some(hio::hstderr()?);
        }

        HSTDERR.as_mut().unwrap().write_fmt(args).map_err(drop)
    });
}
