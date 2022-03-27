#![no_std]

use core::fmt::Debug;
pub use minitest_macros::tests;

/// Private implementation details used by the proc macro.
#[doc(hidden)]
pub mod export;

mod sealed {
    pub trait Sealed {}
    impl Sealed for () {}
    impl<T, E> Sealed for Result<T, E> {}
}

/// Indicates whether a test succeeded or failed.
///
/// This is comparable to the `Termination` trait in libstd, except stable and tailored towards the
/// needs of defmt-test. It is implemented for `()`, which always indicates success, and `Result`,
/// where `Ok` indicates success.
pub trait TestOutcome: Debug + sealed::Sealed {
    fn is_success(&self) -> bool;
}

impl TestOutcome for () {
    fn is_success(&self) -> bool {
        true
    }
}

impl<T: Debug, E: Debug> TestOutcome for Result<T, E> {
    fn is_success(&self) -> bool {
        self.is_ok()
    }
}

#[macro_export]
macro_rules! log {
    ($s:literal $(, $x:expr)* $(,)?)  => {
        {
            #[cfg(feature = "semihosting")]
            ::cortex_m_semihosting::hprintln!($s $(, $x)*);
            #[cfg(feature = "rtt")]
            ::rtt_target::rprintln!($s $(, $x)*);
            #[cfg(not(any(feature = "semihosting", feature="rtt")))]
            let _ = ($( & $x ),*);
        }
    };
}

/// Stop all tests without failure.
pub fn exit() -> ! {
    #[cfg(feature = "rtt")]
    cortex_m::asm::bkpt();
    #[cfg(feature = "semihosting")]
    cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_SUCCESS);

    unreachable!()
}

/// Stop all tests and report a failure.
pub fn fail() -> ! {
    #[cfg(feature = "rtt")]
    cortex_m::asm::udf();
    #[cfg(feature = "semihosting")]
    cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_FAILURE);

    #[cfg(not(feature = "rtt"))]
    unreachable!()
}
