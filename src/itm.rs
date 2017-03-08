//! Instrumentation Trace Macrocell

use core::fmt;

struct Itm {
    port: u8,
}

impl Itm {
    fn write_all(&self, buffer: &[u8]) {
        let stim =
            unsafe { &(*::peripheral::ITM.get()).stim[self.port as usize] };

        for byte in buffer {
            while !stim.is_fifo_ready() {}
            stim.write_u8(*byte);
        }
    }
}

impl fmt::Write for Itm {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes());
        Ok(())
    }
}

/// Writes `fmt::Arguments` to the ITM `port`
pub fn write_fmt(port: u8, args: fmt::Arguments) {
    use core::fmt::Write;

    Itm { port }.write_fmt(args).ok();
}

/// Writes a string to the ITM `port`
pub fn write_str(port: u8, string: &str) {
    Itm { port }.write_all(string.as_bytes())
}
