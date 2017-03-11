/// Macro for printing to the **host's** standard stderr
#[macro_export]
macro_rules! ehprint {
    ($s:expr) => {
        $crate::semihosting:::io:ewrite_str($s);
    };
    ($($arg:tt)*) => {
        $crate::semihosting::io::ewrite_fmt(format_args!($($arg)*));
    };
}

/// Macro for printing to the **host's** standard error, with a newline.
#[macro_export]
macro_rules! ehprintln {
    () => (ehprint!("\n"));
    ($fmt:expr) => {
        ehprint!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)*) => {
        ehprint!(concat!($fmt, "\n"), $($arg)*);
    };
}

/// Macro for printing to the **host's** standard output
#[macro_export]
macro_rules! hprint {
    ($s:expr) => {
        $crate::semihosting::io::write_str($s);
    };
    ($($arg:tt)*) => {
        $crate::semihosting::io::write_fmt(format_args!($($arg)*));
    };
}

/// Macro for printing to the **host's** standard output, with a newline.
#[macro_export]
macro_rules! hprintln {
    () => {
        hprint!("\n");
    };
    ($fmt:expr) => {
        hprint!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)*) => {
        hprint!(concat!($fmt, "\n"), $($arg)*);
    };
}

/// Macro for sending a formatted string through an ITM channel
#[macro_export]
macro_rules! iprint {
    ($channel:expr, $s:expr) => {
        $crate::itm::write_str($channel, $s);
    };
    ($channel:expr, $($arg:tt)*) => {
        $crate::itm::write_fmt($channel, format_args!($($arg)*));
    };
}

/// Macro for sending a formatted string through an ITM channel, with a newline.
#[macro_export]
macro_rules! iprintln {
    ($channel:expr) => {
        iprint!($channel, "\n");
    };
    ($channel:expr, $fmt:expr) => {
        iprint!($channel, concat!($fmt, "\n"));
    };
    ($channel:expr, $fmt:expr, $($arg:tt)*) => {
        iprint!($channel, concat!($fmt, "\n"), $($arg)*);
    };
}
