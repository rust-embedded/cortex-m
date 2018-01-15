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

/// Macro to create a mutable reference to a statically allocated value
///
/// This macro returns a value with type `Option<&'static mut $ty>`. `Some($expr)` will be returned
/// the first time the macro is executed; further calls will return `None`. To avoid `unwrap`ping a
/// `None` variant the caller must ensure that the macro is called from a function that's executed
/// at most once in the whole lifetime of the program.
///
/// # Example
///
/// ``` no_run
/// #[macro_use(singleton)]
/// extern crate cortex_m;
///
/// fn main() {
///     // OK if `main` is executed only once
///     let x: &'static mut bool = singleton!(: bool = false).unwrap();
///
///     let y = alias();
///     // BAD this second call to `alias` will definitively `panic!`
///     let y_alias = alias();
/// }
///
/// fn alias() -> &'static mut bool {
///     singleton!(: bool = false).unwrap()
/// }
/// ```
#[macro_export]
macro_rules! singleton {
    (: $ty:ty = $expr:expr) => {
        $crate::interrupt::free(|_| unsafe {
            static mut USED: bool = false;
            static mut VAR: $ty = $expr;

            if USED {
                None
            } else {
                USED = true;
                let var: &'static mut _ = &mut VAR;
                Some(var)
            }
        })
    }
}
