# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.6.2] - 2018-09-09

### Fixed

- Worked around a Cargo limitation that broke builds that depend on `rand`.

- Updated the documentation link in the README to point to working docs.

## [v0.6.1] - 2018-09-06

### Changed

- Produce a better error message if two (or more) copies of `cortex-m-rt` are
  going to be linked into a binary.

## [v0.6.0] - 2018-09-06

### Changed

- [breaking-change] the `entry!`, `pre_init!` and `exception!` macros have been
  replaced with attributes: `#[entry]`, `#[pre_init]` and `#[exception]`,
  respectively. This also changes the toolchain requirement to 1.30-beta or
  newer.

## [v0.5.3] - 2018-08-27

### Changed

- This crate no longer depends on `arm-none-eabi-gcc`.

## [v0.5.2] - 2018-08-11

### Added

* A `pre_init!` macro and related functionality to run a function immediately
  after reset, before memory initialisation

### Changed

- The `entry!` and `exception!` macros now also accept a closure instead of a path.

- `DefaultHandler` and `UserHardFault` now default to an infinite loop if left undefined.

### Fixed

* Linked script modified to correctly detect `FLASH` overflow caused by `.data`

## [v0.5.1] - 2018-05-14

### Fixed

- A recompilation problem where this `cortex-m-rt` would be recompiled every time `cargo build` is
  invoked.

## [v0.5.0] - 2018-05-12

### Added

- An `entry!` macro to set the entry point of the program.

- A `heap_start` function that returns a pointer into the start of the heap region.

- A `device` feature. When disabled this crate provides the interrupt vectors; when enabled the
  interrupt vectors are expected to be provided by another crate. Read the documentation for
  details.

### Changed

- This crate now compiles on the beta and stable channels.

- [breaking-change] this crate now requires `arm-none-eabi-gcc` to be installed and available in
  `$PATH` to compile.

- [breaking-change] the `start` lang item has been removed. The standard `main` interface won't
  work. Instead use `#![no_main]` and the `entry!` macro. See documentation for details.

- [breaking-change] the `default_handler!` macro has been merged into the `exception!` macro. Use
  `exception!(*, ..)` to set the default exception handler.

- [breaking-change] there's no weak default handler so a default handler must be defined by the
  application, or one of its dependencies.

- [breaking-change] the syntax of the third argument of the `exception!` handler has changed. See
  the documentation of the macro for details.

- [breaking-change] the exception names that the `exception!` macro accepts has changed to match the
  CMSIS specification. See the documentation of the macro for the list of names it accepts.

- [breaking-change] The number of symbol interfaces has been reduced. Check the advanced section of
  the documentation for details.

## [v0.4.0] - 2018-04-09

### Added

- LLD support. The linker script provided by this crate has been tweaked to support both LLD and GNU
  LD. To use LLD as a linker change `.cargo/config` to look like this:

``` diff
 [target.thumbv7m-none-eabi]
 rustflags = [
   "-C", "link-arg=-Tlink.x",
-  "-C", "linker=arm-none-eabi-ld",
-  "-Z", "linker-flavor=ld",
+  "-C", "linker=lld",
+  "-Z", "linker-flavor=ld.lld",
 ]
```

### Removed

- [breaking-change] Stack overflow protection has been removed. Unfortunately, supporting this
  feature produces totally wrong `arm-none-eabi-size` reports when LLD is used to link the
  program. If you need the stack overflow protection feature you can continue to use version
  v0.3.13+.

- [breaking-change] The "abort-on-panic" Cargo feature, which provided a `panic_fmt` implementation,
  has been removed. If you were using this feature you can instead use a [panic implementation
  crate][panic-impl].

[panic-impl]: https://crates.io/keywords/panic-impl

## [v0.3.15] - 2018-04-08

### Fixed

- Support the newest nightly

## [v0.3.14] - 2018-04-01

### Fixed

- `dev` channel support

## [v0.3.13] - 2018-02-17

### Added

- Fictitious `.stack` and `.heap` linker sections that represent the locations of the stack and the
  heap in RAM. You can visualize these linker sections by running `arm-none-eabi-size -Ax` over your
  binary.

- Zero cost stack overflow protection when you use the `cortex-m-rt-ld` linker. Check documentation
  for details.

- A `_heap_size` symbol that indicates how large the heap is. This symbol is only used when
  `cortex-m-rt-ld` is used as a linker.

## [v0.3.12] - 2018-01-17

### Fixed

- Support for recent nightlies.

## [v0.3.11] - 2018-01-17 - YANKED

### Changed

- Dynamically support recent nightlies, which have the `termination` lang item, and
  nightly-2017-09-22, which doesn't. That nightly version is used by the docs.rs builder. Supporting
  that version instead of rejecting it ensures this crate and its reverse-dependencies will get
  their documentation built by the docs.rs service.

## [v0.3.10] - 2018-01-17 - YANKED

### Removed

- The nightly date check from build script that improved error messages for users of old,
  unsupported nightlies. Unfortunately the check was preventing this crate and reverse-dependencies
  from getting their documentation build on docs.rs

## [v0.3.9] - 2018-01-07

### Fixed

- `cargo doc` warnings

## [v0.3.8] - 2017-12-29

### Added

- `Termination` lang item

### Changed

- The `start` lang item to match the new signature

## [v0.3.7] - 2017-12-23

### Added

- Support for overriding the DEBUG_MONITOR exception handler on ARMv7-M.

## [v0.3.6] - 2017-10-03

### Fixed

- Builds with multiple codegen units by forcing the linker to look harder for the exceptions vector
  table.

## [v0.3.5] - 2017-07-21

### Fixed

- Remove duplication of default exception handlers. This saves 32 bytes of Flash
  memory (.text).

## [v0.3.4] - 2017-07-19

### Changed

- Align the end of .rodata to a 4-byte boundary. With this the sections that
  will go into Flash memory will be 4 byte aligned at the start and at the
  end. Which seems to be required (?) by Cortex-M0 devices.

- .bss and .data are now padded so their sizes are multiple of 4 bytes. This
  improves the output of `objdump`; before, the output showed "Address
  0x20000004 is out of bounds".

- Linking now aborts if any of the input files contains a .got section. Dynamic
  relocations are not supported and Rust code is not relocatable by default.
  This error only occurs if C code that was compiled with the -fPIC flag is
  linked in. The error message will tell the user how to compile their C code
  without -fPIC.

## [v0.3.3] - 2017-07-14

### Changed

- Updated the documentation: it's no longer necessary to use the
  compiler-builtins repository since that crate landed in rust-lang/rust and
  it's now available in the `rust-src` component.

## [v0.3.2] - 2017-07-07

### Changed

- Tweaked documentation

## [v0.3.1] - 2017-07-07

### Fixed

- A warning when compiling for x86_64 and the "abort-on-panic" feature is
  enabled.

## [v0.3.0] - 2017-07-07

### Added

- A `default_handler!` macro to override the default exception handler.

- An `exception!` macro to override the handler for a particular exception.

### Changed

- The FPU will now be enabled before `main` if the target has FPU support.

- [breaking-change] the features "panic-over-itm" and "panic-over-semihosting"
  has been removed. the `panic_fmt` language item is now *not* included by
  default. An opt-in feature named "abort-on-panic" can be enabled to make this
  crate provide a `panic_fmt` implementation that simply aborts.

- [breaking-change] The sections `.rodata.{exceptions,interrupts}` have been
  renamed to `.vector_table.{exceptions,interrupts}`. This break the old
  mechanism for registering exceptions (`static EXCEPTIONS`); use the new ones:
  `default_handler!` and `exception!`.

- The `_stack_start` is now optional in the `memory.x` file. If unspecified its
  value will be set to `ORIGIN(RAM) + LENGTH(RAM)`.

## [v0.2.4] - 2017-06-03

### Added

- A non-allocatable `.stlog` section to support the [`stlog`] logging framework.

[`stlog`]: https://crates.io/crates/stlog

## [v0.2.3] - 2017-05-30

### Added

- A `_stext` symbol which can be specified in the linker script to customize the
  location of the `.text` section. If not specified the `.text` section will be
  placed right after the `.vector_table` section.

## [v0.2.2] - 2017-05-27

### Added

- A `_sheap` symbol where the heap can be located.

### Changed

- The linker sections have renamed / reorder to make `arm-none-eabi-size -A`
  more useful. You'll now see something like this:

```
$ arm-none-eabi-size -A hello
hello  :
section                size        addr
.vector_table          1024   134217728
.text                   288   134218752
.rodata                  14   134219040
```

- `cortex-m-rt::reset_handler` is now the entry point of all programs that link
  to `cortex-m-rt`. This makes GDB's `load` command work correctly. It will now
  set the Program Counter to `reset_handler` after flashing the program so
  there's no need to reset the microcontroller after flashing.

- Renamed `__exceptions` and `__interrupts` symbols, which are only used
  internally, to `_eexceptions` and `_einterrupts` respectively for consistency.

### Fixed

- Include input `.text` and `.rodata` sections (note: no suffix as in
  `.text.foo`) in the output file. (C) Code compiled without the equivalent
  `-ffunction-sections` / `-fdata-sections` may place stuff in those unsuffixed
  sections.

## [v0.2.1] - 2017-05-07

### Fixed

- Do not load the `.debug_gdb_script` section in flash. It's only needed for
  debugging.

## [v0.2.0] - 2017-04-27

### Changed

- [breaking-change] the `_stack_start` symbol is now required and must be
  provided in the `memory.x` file when using the "linker-script" feature. This
  symbol indicates where in memory the call stack will be allocated.

## [v0.1.3] - 2017-04-25

### Fixed

- A `rustdoc` warning

## [v0.1.2] - 2017-04-22

### Changed

- Unclutter the `reset_handler` function for a better debugging experience.

## [v0.1.1] - 2017-04-15

### Changed

- Improved linker error messages

## v0.1.0 - 2017-04-12

Initial release

[Unreleased]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.6.2...HEAD
[v0.6.2]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.6.1...v0.6.2
[v0.6.1]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.6.0...v0.6.1
[v0.6.0]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.5.3...v0.6.0
[v0.5.3]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.5.2...v0.5.3
[v0.5.2]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.5.1...v0.5.2
[v0.5.1]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.5.0...v0.5.1
[v0.5.0]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.15...v0.4.0
[v0.3.15]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.14...v0.3.15
[v0.3.14]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.13...v0.3.14
[v0.3.13]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.12...v0.3.13
[v0.3.12]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.11...v0.3.12
[v0.3.11]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.10...v0.3.11
[v0.3.10]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.9...v0.3.10
[v0.3.9]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.8...v0.3.9
[v0.3.8]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.7...v0.3.8
[v0.3.7]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.6...v0.3.7
[v0.3.6]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.5...v0.3.6
[v0.3.5]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.4...v0.3.5
[v0.3.4]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.3...v0.3.4
[v0.3.3]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.2...v0.3.3
[v0.3.2]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.2.4...v0.3.0
[v0.2.4]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.2.3...v0.2.4
[v0.2.3]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.1.3...v0.2.0
[v0.1.3]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/rust-embedded/cortex-m-rt/compare/v0.1.0...v0.1.1
