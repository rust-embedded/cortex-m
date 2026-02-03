//! IMPLEMENTATION DETAILS USED BY MACROS

use core::cell::RefCell;
use core::fmt::{self, Write};

use crate::hio::{self, HostStream};

static HSTDOUT: critical_section::Mutex<RefCell<Option<HostStream>>> =
    critical_section::Mutex::new(RefCell::new(None));

pub fn hstdout_str(s: &str) {
    critical_section::with(|cs| {
        let mut hstdout_opt = HSTDOUT.borrow_ref_mut(cs);
        if hstdout_opt.is_none() {
            if let Ok(hstdout) = hio::hstdout() {
                hstdout_opt.replace(hstdout);
            } else {
                return;
            }
        }
        let hstdout = hstdout_opt.as_mut().unwrap();
        let _ = hstdout.write_str(s);
    });
}

pub fn hstdout_fmt(args: fmt::Arguments) {
    critical_section::with(|cs| {
        let mut hstdout_opt = HSTDOUT.borrow_ref_mut(cs);
        if hstdout_opt.is_none() {
            if let Ok(hstdout) = hio::hstdout() {
                hstdout_opt.replace(hstdout);
            } else {
                return;
            }
        }
        let hstdout = hstdout_opt.as_mut().unwrap();
        let _ = hstdout.write_fmt(args);
    });
}

static HSTDERR: critical_section::Mutex<RefCell<Option<HostStream>>> =
    critical_section::Mutex::new(RefCell::new(None));

pub fn hstderr_str(s: &str) {
    critical_section::with(|cs| {
        let mut hstderr_opt = HSTDERR.borrow_ref_mut(cs);
        if let Ok(hstderr) = hio::hstderr() {
            hstderr_opt.replace(hstderr);
        } else {
            return;
        }
        let hstderr = hstderr_opt.as_mut().unwrap();
        let _ = hstderr.write_str(s);
    });
}

pub fn hstderr_fmt(args: fmt::Arguments) {
    critical_section::with(|cs| {
        let mut hstderr_opt = HSTDERR.borrow_ref_mut(cs);
        if let Ok(hstderr) = hio::hstderr() {
            hstderr_opt.replace(hstderr);
        } else {
            return;
        }
        let hstderr = hstderr_opt.as_mut().unwrap();
        let _ = hstderr.write_fmt(args);
    });
}
