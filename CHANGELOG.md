# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- `Exception` a enumeration of the kind of exceptions the processor can service.
  There's also a `Exception::current` constructor that returns the `Exception`
  that's currently being serviced.

## [v0.1.5]

### Added

- `interrupt::Mutex`, a "mutex" based on critical sections.

### Changed

- The closure that `interrupt::free` takes can now return a value.

## [v0.1.4]

### Added

- `asm::nop`, a wrapper over the NOP instruction

## [v0.1.3]

### Added

- a StackFrame data structure

## [v0.1.2] - 2016-10-04

### Fixed

- Read/write Operations on registers (lr, cr, msp, etc.) which were reversed.

## [v0.1.1] - 2016-10-03 [YANKED]

### Changed

- Small, non user visible change to make this crate compile further for $HOST (e.g. x86_64) with the
  goal of making it possible to test, on the HOST, downstream crates that depend on this one.

## v0.1.0 - 2016-09-27 [YANKED]

### Added

- Functions to access core peripherals like NVIC, SCB and SysTick.
- Functions to access core registers like CONTROL, MSP and PSR.
- Functions to enable/disable interrupts
- Functions to get the vector table
- Wrappers over miscellaneous instructions like `bkpt`

[Unreleased]: https://github.com/japaric/rustc-cfg/compare/v0.1.5...HEAD
[v0.1.5]: https://github.com/japaric/rustc-cfg/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/japaric/rustc-cfg/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/japaric/rustc-cfg/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/japaric/rustc-cfg/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/rustc-cfg/compare/v0.1.0...v0.1.1
