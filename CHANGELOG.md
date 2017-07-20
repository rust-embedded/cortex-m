# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.3.1] - 2017-07-20

### Changed

- `{basepri,basepri_max}::write` are now compiler barriers for the same reason
  that `interrupt::{disable,enable}` are: they are used to create critical
  sections.

## [v0.3.0] - 2017-07-07

### Changed

- [breaking-change] Renamed `StackedRergisters` to `ExceptionFrame` to better
  reflect the ARM documentation.

- [breaking-change] Renamed the variants of `Exception` to better match the
  ARM documentation.

- [breaking-change] Renamed `Exception::current` to `Exception::active` and
  changed the signature to return `None` when no exception is being serviced.

- Moved bits non specific to the Cortex-M architecture into the [`bare-metal`]
  crate with the goal of sharing code between this crate and crates tailored for
  other (microcontroller) architectures.

[`bare-metal`]: https://crates.io/crates/bare-metal

### Removed

- [breaking-change] The `ctxt` module along with the exception "tokens" in the
  `exception` module. The `cortex-m-rt` crate v0.3.0 provides a more ergonomic
  mechanism to add state to interrupts / exceptions; replace your uses of
  `Local` with that.

- [breaking-change] `default_handler`, `DEFAULT_HANDLERS` and `Handlers` from
  the `exception` module as well as `Reserved` from the root of the crate.
  `cortex-m-rt` v0.3.0 provides a mechanism to override exceptions and the
  default exception handler. Change your use of these `Handlers` and others to
  that.

### Fixed

- `interrupt::{enable,disable}` are now compiler barriers. The compiler should
  not reorder code around these function calls for memory safety; that is the
  case now.

## [v0.2.11] - 2017-06-16

### Added

- An API to maintain the different caches (DCache, ICache) on Cortex M7 devices.

### Fixed

- the definition of the `ehprint!` macro.
- the implementation of the FPU API.

## [v0.2.10] - 2017-06-05

### Added

- Functions for the instructions DMB, ISB and DSB

### Changed

- All the functions in the `asm` module are now `inline(always)`

## [v0.2.9] - 2017-05-30

### Fixed

- A bug in `itm::write_all` where it would ignore the length of the buffer and
  serialize contents that come after the buffer.

## [v0.2.8] - 2017-05-30 - YANKED

### Added

- An `itm::write_aligned` function to write 4 byte aligned buffers to an ITM
  port. This function is faster than `itm::write_all` for small buffers but
  requires the buffer to be aligned.

## [v0.2.7] - 2017-05-23

### Added

- `Dwt.enable_cycle_counter`

## [v0.2.6] - 2017-05-08

### Fixed

- [breaking-change]. MEMORY UNSAFETY. `Mutex` could be used as a channel to send
  interrupt tokens from one interrupt to other thus breaking the context `Local`
  abstraction. See reproduction case below. This has been fixed by making
  `Mutex` `Sync` only if the protected data is `Send`.

``` rust
#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f30x;

use core::cell::RefCell;

use cortex_m::ctxt::Local;
use cortex_m::interrupt::Mutex;
use stm32f30x::interrupt::{self, Exti0, Exti1};

fn main() {
    // ..

    // trigger exti0
    // then trigger exti0 again
}

static CHANNEL: Mutex<RefCell<Option<Exti0>>> = Mutex::new(RefCell::new(None));
// Supposedly task *local* data
static LOCAL: Local<i32, Exti0> = Local::new(0);

extern "C" fn exti0(mut ctxt: Exti0) {
    static FIRST: Local<bool, Exti0> = Local::new(true);

    let first = *FIRST.borrow(&ctxt);

    // toggle
    if first {
        *FIRST.borrow_mut(&mut ctxt) = false;
    }

    if first {
        cortex_m::interrupt::free(
            |cs| {
                let channel = CHANNEL.borrow(cs);

                // BAD: transfer interrupt token to another interrupt
                *channel.borrow_mut() = Some(ctxt);
            },
        );

        return;
    }
    let _local = LOCAL.borrow_mut(&mut ctxt);

    // ..

    // trigger exti1 here

    // ..

    // `LOCAL` mutably borrowed up to this point
}

extern "C" fn exti1(_ctxt: Exti1) {
    cortex_m::interrupt::free(|cs| {
        let channel = CHANNEL.borrow(cs);
        let mut channel = channel.borrow_mut();

        if let Some(mut other_task) = channel.take() {
            // BAD: `exti1` has access to `exti0`'s interrupt token
            // so it can now mutably access local while `exti0` is also using it
            let _local = LOCAL.borrow_mut(&mut other_task);
        }
    });
}

#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: interrupt::Handlers = interrupt::Handlers {
    Exti0: exti0,
    Exti1: exti1,
    ..interrupt::DEFAULT_HANDLERS
};
```

## [v0.2.5] - 2017-05-07 - YANKED

### Added

- Higher level API for the SysTick and FPU peripherals

### Fixed

- [breaking-change]. MEMORY UNSAFETY. `interrupt::enable` was safe to call
  inside an `interrupt::free` critical section thus breaking the preemption
  protection. The `interrupt::enable` method is now `unsafe`.

## [v0.2.4] - 2017-04-20 - YANKED

### Fixed

- [breaking-change]. MEMORY UNSAFETY. `interrupt::free` leaked the critical
  section making it possible to access a `Mutex` when interrupts are enabled
  (see below). This has been fixed by changing the signature of
  `interrupt::free`.

``` rust
static FOO: Mutex<bool> = Mutex::new(false);

fn main() {
    let cs = cortex_m::interrupt::free(|cs| cs);
    // interrupts are enabled at this point
    let foo = FOO.borrow(&cs);
}
```

## [v0.2.3] - 2017-04-11 - YANKED

### Fixed

- [breaking-change]. MEMORY UNSAFETY. Some concurrency models that use "partial"
  critical sections (cf. BASEPRI) can be broken by changing the priority of
  interrupts or by changing BASEPRI in some scenarios. For this reason
  `NVIC.set_priority` and `register::basepri::write` are now `unsafe`.

## [v0.2.2] - 2017-04-08 - YANKED

### Fixed

- [breaking-change]. MEMORY UNSAFETY. The `Mutex.borrow_mut` method has been
  removed as it can be used to bypass Rust's borrow checker and get, for
  example, two mutable references to the same data.

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

## [v0.1.1] - 2016-10-03 - YANKED

### Changed

- Small, non user visible change to make this crate compile further for $HOST (e.g. x86_64) with the
  goal of making it possible to test, on the HOST, downstream crates that depend on this one.

## v0.1.0 - 2016-09-27 - YANKED

### Added

- Functions to access core peripherals like NVIC, SCB and SysTick.
- Functions to access core registers like CONTROL, MSP and PSR.
- Functions to enable/disable interrupts
- Functions to get the vector table
- Wrappers over miscellaneous instructions like `bkpt`

[Unreleased]: https://github.com/japaric/cortex-m/compare/v0.3.1...HEAD
[v0.3.1]: https://github.com/japaric/cortex-m/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/japaric/cortex-m/compare/v0.2.11...v0.3.0
[v0.2.11]: https://github.com/japaric/cortex-m/compare/v0.2.10...v0.2.11
[v0.2.10]: https://github.com/japaric/cortex-m/compare/v0.2.9...v0.2.10
[v0.2.9]: https://github.com/japaric/cortex-m/compare/v0.2.8...v0.2.9
[v0.2.8]: https://github.com/japaric/cortex-m/compare/v0.2.7...v0.2.8
[v0.2.7]: https://github.com/japaric/cortex-m/compare/v0.2.6...v0.2.7
[v0.2.6]: https://github.com/japaric/cortex-m/compare/v0.2.5...v0.2.6
[v0.2.5]: https://github.com/japaric/cortex-m/compare/v0.2.4...v0.2.5
[v0.2.4]: https://github.com/japaric/cortex-m/compare/v0.2.3...v0.2.4
[v0.2.3]: https://github.com/japaric/cortex-m/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/japaric/cortex-m/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/japaric/cortex-m/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/japaric/cortex-m/compare/v0.1.6...v0.2.0
[v0.1.6]: https://github.com/japaric/cortex-m/compare/v0.1.5...v0.1.6
[v0.1.5]: https://github.com/japaric/cortex-m/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/japaric/cortex-m/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/japaric/cortex-m/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/japaric/cortex-m/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/cortex-m/compare/v0.1.0...v0.1.1
