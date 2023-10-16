# Cortex-M crates

This repository contains various crates useful for writing Rust programs
on Cortex-M microcontrollers:

* [`cortex-m`]: CPU peripheral access and intrinsics
* [`cortex-m-rt`]: Startup code and interrupt handling
* [`cortex-m-semihosting`]: Support for semihosting debugging
* [`cortex-m-interrupt-number`]: Shared trait for interacting with peripheral access crates
* [`panic-itm`]: Panic handler that sends messages over the ITM/SWO output
* [`panic-semihosting`]: Panic handler that sends messages over semihosting

[`cortex-m`]: https://crates.io/crates/cortex-m
[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt
[`cortex-m-semihosting`]: https://crates.io/crates/cortex-m-semihosting
[`cortex-m-interrupt-number`]: https://crates.io/crates/cortex-m-interrupt-number
[`panic-itm`]: https://crates.io/crates/panic-itm
[`panic-semihosting`]: https://crates.io/crates/panic-semihosting

This project is developed and maintained by the [Cortex-M team][team].

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Code of Conduct

Contribution to this repository is organized under the terms of the [Rust Code
of Conduct][CoC], the maintainer of this crate, the [Cortex-M team][team],
promises to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-cortex-m-team
