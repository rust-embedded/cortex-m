//! Instrumentation Trace Macrocell

use core::{fmt, ptr, slice};
use peripheral::Stim;

fn round_up_to_multiple_of(x: usize, k: usize) -> usize {
    let rem = x % k;

    if rem == 0 { x } else { x + k - rem }
}

fn round_down_to_multiple_of(x: usize, k: usize) -> usize {
    x - (x % k)
}

unsafe fn split(buffer: &[u8]) -> (&[u8], &[u32], &[u8]) {
    let start = buffer.as_ptr();
    let end = start.offset(buffer.len() as isize);
    let sbody = round_up_to_multiple_of(start as usize, 4);
    let ebody = round_down_to_multiple_of(end as usize, 4);

    let head = slice::from_raw_parts(start, sbody - start as usize);
    let body = slice::from_raw_parts(sbody as *const _, (ebody - sbody) >> 2);
    let tail = slice::from_raw_parts(ebody as *const _, end as usize - ebody);

    (head, body, tail)
}

fn write_bytes(stim: &Stim, bytes: &[u8]) {
    for byte in bytes {
        while !stim.is_fifo_ready() {}
        stim.write_u8(*byte);
    }
}

// NOTE assumes that `bytes` is 32-bit aligned
unsafe fn write_words(stim: &Stim, bytes: &[u32]) {
    let mut p = bytes.as_ptr();
    for _ in 0..bytes.len() {
        while !stim.is_fifo_ready() {}
        stim.write_u32(ptr::read(p));
        p = p.offset(1);
    }
}

struct Port<'p>(&'p Stim);

impl<'p> fmt::Write for Port<'p> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_all(self.0, s.as_bytes());
        Ok(())
    }
}

/// Writes a `buffer` to the ITM `port`
pub fn write_all(port: &Stim, buffer: &[u8]) {
    if buffer.len() < 7 {
        write_bytes(port, buffer);
    } else {
        let (head, body, tail) = unsafe { split(buffer) };
        write_bytes(port, head);
        unsafe { write_words(port, body) }
        write_bytes(port, tail);
    }
}

/// Writes `fmt::Arguments` to the ITM `port`
pub fn write_fmt(port: &Stim, args: fmt::Arguments) {
    use core::fmt::Write;

    Port(port).write_fmt(args).ok();
}

/// Writes a string to the ITM `port`
pub fn write_str(port: &Stim, string: &str) {
    write_all(port, string.as_bytes())
}
