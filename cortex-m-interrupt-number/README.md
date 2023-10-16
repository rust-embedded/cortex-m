# cortex-m-interrupt-number

This crate provides the definition of a trait that is shared between
the `cortex-m` crate and all peripheral access crates (PACs) for
Cortex-M microcontrollers.

The PACs must implement the `InterruptNumber` trait on an enum of possible
interrupts; refer to the `InterruptNumber` [documentation] for more details.

[documentation]: https://docs.rs/cortex-m-interrupt-number
