#![no_std]

/// Trait for enums of target-specific exception numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC) on its enum of available
/// exceptions for a specific device. Alternatively, the `cortex-m` crate provides a default
/// implementation. Each variant must convert to a `usize` of its exception number.
///
/// # Safety
///
/// * This trait must only be implemented on the `cortex-m` crate or on a PAC of a Cortex-M target.
/// * This trait must only be implemented on enums of exceptions.
/// * Each enum variant must represent a distinct value (no duplicates are permitted),
/// * Each enum variant must always return the same value (do not change at runtime).
/// * All the exception numbers must be less than or equal to `MAX_EXCEPTION_NUMBER`.
/// * `MAX_EXCEPTION_NUMBER` must coincide with the highest allowed exception number.
pub unsafe trait ExceptionNumber: Copy {
    /// Highest number assigned to an exception.
    const MAX_EXCEPTION_NUMBER: usize;

    /// Converts an exception to its corresponding number.
    fn number(self) -> usize;

    /// Tries to convert a number to a valid exception.
    fn from_number(value: usize) -> Result<Self, InvalidExceptionNumberError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidExceptionNumberError;

impl core::fmt::Display for InvalidExceptionNumberError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid exception number")
    }
}

impl core::error::Error for InvalidExceptionNumberError {}

/// Trait for enums of target-specific interrupt numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC) on its enum of available
/// interrupts for a specific device. Each variant must convert to a `usize` of its interrupt number.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a Cortex-M target.
/// * This trait must only be implemented on enums of interrupts.
/// * Each enum variant must represent a distinct value (no duplicates are permitted),
/// * Each enum variant must always return the same value (do not change at runtime).
/// * All the interrupt numbers must be less than or equal to `MAX_INTERRUPT_NUMBER`.
/// * `MAX_INTERRUPT_NUMBER` must coincide with the highest allowed interrupt number.
pub unsafe trait InterruptNumber: Copy {
    /// Highest number assigned to an interrupt source.
    const MAX_INTERRUPT_NUMBER: usize;

    /// Converts an interrupt source to its corresponding number.
    fn number(self) -> usize;

    /// Tries to convert a number to a valid interrupt.
    fn from_number(value: usize) -> Result<Self, InvalidInterruptNumberError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidInterruptNumberError;

impl core::fmt::Display for InvalidInterruptNumberError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid interrupt number")
    }
}

impl core::error::Error for InvalidInterruptNumberError {}

/// Trait for enums of priority levels.
///
/// This trait should be implemented by a peripheral access crate (PAC) on its enum of available
/// priority numbers for a specific device. Each variant must convert to a `usize` of its priority level.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a Cortex-M target.
/// * This trait must only be implemented on enums of priority levels.
/// * Each enum variant must represent a distinct value (no duplicates are permitted).
/// * Each enum variant must always return the same value (do not change at runtime).
/// * All the priority level numbers must be less than or equal to `MAX_PRIORITY_NUMBER`.
/// * `MAX_PRIORITY_NUMBER` must coincide with the highest allowed priority number.
pub unsafe trait PriorityNumber: Copy {
    /// Number assigned to the highest priority level.
    const MAX_PRIORITY_NUMBER: usize;

    /// Converts a priority level to its corresponding number.
    fn number(self) -> usize;

    /// Tries to convert a number to a valid priority level.
    fn from_number(value: usize) -> Result<Self, InvalidPriorityNumberError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidPriorityNumberError;

impl core::fmt::Display for InvalidPriorityNumberError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid priority number")
    }
}

impl core::error::Error for InvalidPriorityNumberError {}

/// Trait for enums of core identifiers.
///
/// This trait should be implemented by a peripheral access crate (PAC) on its enum of available
/// cores for a specific device. Each variant must convert to a `usize` of its core ID number.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a Cortex-M target.
/// * This trait must only be implemented on enums of core IDs.
/// * Each enum variant must represent a distinct value (no duplicates are permitted),
/// * Each anum variant must always return the same value (do not change at runtime).
/// * All the core ID numbers must be less than or equal to `MAX_CORE_ID_NUMBER`.
/// * `MAX_CORE_ID_NUMBER` must coincide with the highest allowed core ID number.
pub unsafe trait CoreIdNumber: Copy {
    /// Highest number assigned to a core.
    const MAX_CORE_ID_NUMBER: usize;

    /// Converts a Core ID to its corresponding number.
    fn number(self) -> usize;

    /// Tries to convert a number to a valid Core ID.
    fn from_number(value: usize) -> Result<Self, InvalidCoreIdNumberError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidCoreIdNumberError;

impl core::fmt::Display for InvalidCoreIdNumberError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid core ID number")
    }
}

impl core::error::Error for InvalidCoreIdNumberError {}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Exception {
        E1 = 1,
        E3 = 3,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Interrupt {
        I1 = 1,
        I2 = 2,
        I4 = 4,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Priority {
        P0 = 0,
        P1 = 1,
        P2 = 2,
        P3 = 3,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum CoreId {
        H0 = 0,
        H1 = 1,
        H2 = 2,
    }

    unsafe impl ExceptionNumber for Exception {
        const MAX_EXCEPTION_NUMBER: usize = Self::E3 as usize;

        #[inline]
        fn number(self) -> usize {
            self as _
        }

        #[inline]
        fn from_number(number: usize) -> Result<Self, InvalidExceptionNumberError> {
            match number {
                1 => Ok(Exception::E1),
                3 => Ok(Exception::E3),
                _ => Err(InvalidExceptionNumberError),
            }
        }
    }

    unsafe impl InterruptNumber for Interrupt {
        const MAX_INTERRUPT_NUMBER: usize = Self::I4 as usize;

        #[inline]
        fn number(self) -> usize {
            self as _
        }

        #[inline]
        fn from_number(number: usize) -> Result<Self, InvalidInterruptNumberError> {
            match number {
                1 => Ok(Interrupt::I1),
                2 => Ok(Interrupt::I2),
                4 => Ok(Interrupt::I4),
                _ => Err(InvalidInterruptNumberError),
            }
        }
    }

    unsafe impl PriorityNumber for Priority {
        const MAX_PRIORITY_NUMBER: usize = Self::P3 as usize;

        #[inline]
        fn number(self) -> usize {
            self as _
        }

        #[inline]
        fn from_number(number: usize) -> Result<Self, InvalidPriorityNumberError> {
            match number {
                0 => Ok(Priority::P0),
                1 => Ok(Priority::P1),
                2 => Ok(Priority::P2),
                3 => Ok(Priority::P3),
                _ => Err(InvalidPriorityNumberError),
            }
        }
    }

    unsafe impl CoreIdNumber for CoreId {
        const MAX_CORE_ID_NUMBER: usize = Self::H2 as usize;

        #[inline]
        fn number(self) -> usize {
            self as _
        }

        #[inline]
        fn from_number(number: usize) -> Result<Self, InvalidCoreIdNumberError> {
            match number {
                0 => Ok(CoreId::H0),
                1 => Ok(CoreId::H1),
                2 => Ok(CoreId::H2),
                _ => Err(InvalidCoreIdNumberError),
            }
        }
    }

    #[test]
    fn check_exception_enum() {
        assert_eq!(Exception::E1.number(), 1);
        assert_eq!(Exception::E3.number(), 3);

        assert_eq!(Exception::from_number(0), Err(InvalidExceptionNumberError));
        assert_eq!(Exception::from_number(1), Ok(Exception::E1));
        assert_eq!(Exception::from_number(2), Err(InvalidExceptionNumberError));
        assert_eq!(Exception::from_number(3), Ok(Exception::E3));
        assert_eq!(Exception::from_number(4), Err(InvalidExceptionNumberError));
    }

    #[test]
    fn check_interrupt_enum() {
        assert_eq!(Interrupt::I1.number(), 1);
        assert_eq!(Interrupt::I2.number(), 2);
        assert_eq!(Interrupt::I4.number(), 4);

        assert_eq!(Interrupt::from_number(0), Err(InvalidInterruptNumberError));
        assert_eq!(Interrupt::from_number(1), Ok(Interrupt::I1));
        assert_eq!(Interrupt::from_number(2), Ok(Interrupt::I2));
        assert_eq!(Interrupt::from_number(3), Err(InvalidInterruptNumberError));
        assert_eq!(Interrupt::from_number(4), Ok(Interrupt::I4));
        assert_eq!(Interrupt::from_number(5), Err(InvalidInterruptNumberError));
    }

    #[test]
    fn check_priority_enum() {
        assert_eq!(Priority::P0.number(), 0);
        assert_eq!(Priority::P1.number(), 1);
        assert_eq!(Priority::P2.number(), 2);
        assert_eq!(Priority::P3.number(), 3);

        assert_eq!(Priority::from_number(0), Ok(Priority::P0));
        assert_eq!(Priority::from_number(1), Ok(Priority::P1));
        assert_eq!(Priority::from_number(2), Ok(Priority::P2));
        assert_eq!(Priority::from_number(3), Ok(Priority::P3));
        assert_eq!(Priority::from_number(4), Err(InvalidPriorityNumberError));
    }

    #[test]
    fn check_core_id_enum() {
        assert_eq!(CoreId::H0.number(), 0);
        assert_eq!(CoreId::H1.number(), 1);
        assert_eq!(CoreId::H2.number(), 2);

        assert_eq!(CoreId::from_number(0), Ok(CoreId::H0));
        assert_eq!(CoreId::from_number(1), Ok(CoreId::H1));
        assert_eq!(CoreId::from_number(2), Ok(CoreId::H2));
        assert_eq!(CoreId::from_number(3), Err(InvalidCoreIdNumberError));
    }
}
