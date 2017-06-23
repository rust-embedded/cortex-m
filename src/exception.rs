//! Exceptions

#![allow(non_camel_case_types)]

/// Enumeration of all exceptions
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Vector {
    /// Fault or system exception
    Exception(Exception),
    /// An interrupt
    Interrupt(u8),
    // Unreachable variant
    #[doc(hidden)]
    Reserved,
}

impl Vector {
    /// Returns the kind of exception that's currently being serviced
    pub fn active() -> Option<Vector> {
        // NOTE(safe) atomic read
        let icsr = unsafe { (*::peripheral::SCB.get()).icsr.read() };
        if icsr == 0 {
            return None;
        }

        Some(match icsr as u8 {
            2 => Vector::Exception(Exception::NMI),
            3 => Vector::Exception(Exception::HARD_FAULT),
            4 => Vector::Exception(Exception::MEN_MANAGE),
            5 => Vector::Exception(Exception::BUS_FAULT),
            6 => Vector::Exception(Exception::USAGE_FAULT),
            11 => Vector::Exception(Exception::SVCALL),
            14 => Vector::Exception(Exception::PENDSV),
            15 => Vector::Exception(Exception::SYS_TICK),
            n if n >= 16 => Vector::Interrupt(n - 16),
            _ => Vector::Reserved,
        })
    }
}

/// Registers stacked during an exception
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct StackedRegisters {
    /// (General purpose) Register 0
    pub r0: u32,
    /// (General purpose) Register 1
    pub r1: u32,
    /// (General purpose) Register 2
    pub r2: u32,
    /// (General purpose) Register 3
    pub r3: u32,
    /// (General purpose) Register 12
    pub r12: u32,
    /// Linker Register
    pub lr: u32,
    /// Program Counter
    pub pc: u32,
    /// Program Status Register
    pub xpsr: u32,
}

#[macro_export]
macro_rules! default_handler {
    ($f:ident, local: {
        $($lvar:ident:$lty:ident = $lval:expr;)*
    }) => {
        #[allow(non_snake_case)]
        mod DEFAULT_HANDLER {
            pub struct Locals {
                $(
                    pub $lvar: $lty,
                )*
            }
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn DEFAULT_HANDLER() {
            static mut LOCALS: self::DEFAULT_HANDLER::Locals =
                self::DEFAULT_HANDLER::Locals {
                    $(
                        $lvar: $lval,
                    )*
                };

            // type checking
            let f: fn(&mut self::DEFAULT_HANDLER::Locals) = $f;
            f(unsafe { &mut LOCALS });
        }
    };
    ($f:ident) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn DEFAULT_HANDLER() {
            // type checking
            let f: fn() = $f;
            f();
        }
    }
}

/// Fault and system exceptions
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Exception {
    /// Non-maskable interrupt
    NMI,
    /// All class of fault.
    HARD_FAULT,
    /// Memory management.
    MEN_MANAGE,
    /// Pre-fetch fault, memory access fault.
    BUS_FAULT,
    /// Undefined instruction or illegal state.
    USAGE_FAULT,
    /// System service call via SWI instruction
    SVCALL,
    /// Pendable request for system service
    PENDSV,
    /// System tick timer
    SYS_TICK,
}

#[macro_export]
macro_rules! exception {
    ($NAME:ident, $f:path, local: {
        $($lvar:ident:$lty:ident = $lval:expr;)*
    }) => {
        #[allow(non_snake_case)]
        mod $NAME {
            pub struct Locals {
                $(
                    pub $lvar: $lty,
                )*
            }
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn $NAME() {
            // check that the handler exists
            let _ = $crate::exception::Exception::$NAME;

            static mut LOCALS: self::$NAME::Locals = self::$NAME::Locals {
                $(
                    $lvar: $lval,
                )*
            };

            // type checking
            let f: fn(&mut self::$NAME::Locals) = $f;
            f(unsafe { &mut LOCALS });
        }
    };
    ($NAME:ident, $f:path) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn $NAME() {
            // check that the handler exists
            let _ = $crate::exception::Exception::$NAME;

            // type checking
            let f: fn() = $f;
            f();
        }
    }
}
