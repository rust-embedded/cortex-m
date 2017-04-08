# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.2] - 2017-04-08

### Fixed

- MEMORY SAFETY BUG. The `Mutex.borrow_mut` method has been removed as it can be
  used to bypass Rust's borrow checker and get, for example, two mutable
  references to the same data.

``` rust
static FOO: Mutex<bool> = Mutex::new(false);

fn main() {
    cortex_m::interrupt::free(|mut cs1| {
        cortex_m::interrupt::free(|mut cs2| {
            let foo: &mut bool = FOO.borrow_mut(&mut cs1);
            let and_foo: &mut bool = FOO.borrow_mut(&mut cs2);
        });
    });
}
```

## [v0.2.1] - 2017-03-12 - YANKED

### Changed

- The default exception handler now identifies the exception that's being
  serviced.

## [v0.2.0] - 2017-03-11 - YANKED

### Added

- Semihosting functionality in the `semihosting` module.

- `exception::Handlers` struct that represent the section of the vector table
  that contains the exception handlers.

- A default exception handler

- A high level API for the NVIC peripheral.

- Context local data.

- `borrow`/`borrow_mut` methods to `Mutex` that replace `lock`.

- API and macros to send bytes / (formatted) strings through ITM

### Changed

- [breaking-change] `StackFrame` has been renamed to `StackedRegisters` and
  moved into the `exceptions` module.

- [breaking-change] Core peripherals can now be modified via a `&-` reference
  and are no longer `Sync`.

- [breaking-change] `interrupt::free`'s closure now includes a critical section
  token, `CriticalSection`.

- [breaking-change] the core register API has been revamped for type safety.

- The safety of assembly wrappers like `wfi` and `interrupt::free` has been
  reviewed. In many cases, the functions are no longer unsafe.

- [breaking-change] `bkpt!` has been turned into a function. It no longer
  accepts an immediate value.

### Removed

- `vector_table` and its associated `struct`, `VectorTable`. It's not a good
  idea to give people a simple way to call the exception handlers.

- `Mutex`'s `lock` method as it's unsound. You could use it to get multiple
  `&mut -` references to the wrapped data.

## [v0.1.6] - 2017-01-22

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

[Unreleased]: https://github.com/japaric/cortex-m/compare/v0.2.2...HEAD
[v0.2.2]: https://github.com/japaric/cortex-m/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/japaric/cortex-m/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/japaric/cortex-m/compare/v0.1.6...v0.2.0
[v0.1.6]: https://github.com/japaric/cortex-m/compare/v0.1.5...v0.1.6
[v0.1.5]: https://github.com/japaric/cortex-m/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/japaric/cortex-m/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/japaric/cortex-m/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/japaric/cortex-m/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/cortex-m/compare/v0.1.0...v0.1.1
