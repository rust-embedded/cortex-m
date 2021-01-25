//! Instrumentation Trace Macrocell
//!
//! **NOTE** This module is only available on ARMv7-M and newer

use core::{fmt, mem, ptr, slice};

use aligned::{Aligned, A4};

#[cfg(armv8m_base)]
use crate::peripheral::itm::Stim;
#[cfg(not(armv8m_base))]
use cortex_m_0_7::peripheral::itm::Stim;

// NOTE assumes that `bytes` is 32-bit aligned
#[allow(clippy::missing_inline_in_public_items)]
unsafe fn write_words(stim: &mut Stim, bytes: &[u32]) {
    let mut p = bytes.as_ptr();
    for _ in 0..bytes.len() {
        while !stim.is_fifo_ready() {}
        stim.write_u32(ptr::read(p));
        p = p.offset(1);
    }
}

struct Port<'p>(&'p mut Stim);

impl<'p> fmt::Write for Port<'p> {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_all(self.0, s.as_bytes());
        Ok(())
    }
}

/// Writes a `buffer` to the ITM `port`
#[allow(clippy::cast_ptr_alignment)]
#[allow(clippy::missing_inline_in_public_items)]
#[allow(clippy::transmute_ptr_to_ptr)]
pub fn write_all(port: &mut Stim, buffer: &[u8]) {
    unsafe {
        let mut len = buffer.len();
        let mut ptr = buffer.as_ptr();

        if len == 0 {
            return;
        }

        // 0x01 OR 0x03
        if ptr as usize % 2 == 1 {
            while !port.is_fifo_ready() {}
            port.write_u8(*ptr);

            // 0x02 OR 0x04
            ptr = ptr.offset(1);
            len -= 1;
        }

        // 0x02
        if ptr as usize % 4 == 2 {
            if len > 1 {
                // at least 2 bytes
                while !port.is_fifo_ready() {}
                port.write_u16(ptr::read(ptr as *const u16));

                // 0x04
                ptr = ptr.offset(2);
                len -= 2;
            } else {
                if len == 1 {
                    // last byte
                    while !port.is_fifo_ready() {}
                    port.write_u8(*ptr);
                }

                return;
            }
        }

        write_aligned(port, mem::transmute(slice::from_raw_parts(ptr, len)));
    }
}

/// Writes a 4-byte aligned `buffer` to the ITM `port`
///
/// # Examples
///
/// ``` ignore
/// let mut buffer: Aligned<A4, _> = Aligned([0; 14]);
///
/// buffer.copy_from_slice(b"Hello, world!\n");
///
/// itm::write_aligned(&itm.stim[0], &buffer);
///
/// // Or equivalently
/// itm::write_aligned(&itm.stim[0], &Aligned(*b"Hello, world!\n"));
/// ```
#[allow(clippy::cast_ptr_alignment)]
#[allow(clippy::missing_inline_in_public_items)]
#[allow(clippy::transmute_ptr_to_ptr)]
pub fn write_aligned(port: &mut Stim, buffer: &Aligned<A4, [u8]>) {
    unsafe {
        let len = buffer.len();

        if len == 0 {
            return;
        }

        let split = len & !0b11;
        write_words(
            port,
            slice::from_raw_parts(buffer.as_ptr() as *const u32, split >> 2),
        );

        // 3 bytes or less left
        let mut left = len & 0b11;
        let mut ptr = buffer.as_ptr().add(split);

        // at least 2 bytes left
        if left > 1 {
            while !port.is_fifo_ready() {}
            port.write_u16(ptr::read(ptr as *const u16));

            ptr = ptr.offset(2);
            left -= 2;
        }

        // final byte
        if left == 1 {
            while !port.is_fifo_ready() {}
            port.write_u8(*ptr);
        }
    }
}

/// Writes `fmt::Arguments` to the ITM `port`
#[inline]
pub fn write_fmt(port: &mut Stim, args: fmt::Arguments) {
    use core::fmt::Write;

    Port(port).write_fmt(args).ok();
}

/// Writes a string to the ITM `port`
#[inline]
pub fn write_str(port: &mut Stim, string: &str) {
    write_all(port, string.as_bytes())
}
