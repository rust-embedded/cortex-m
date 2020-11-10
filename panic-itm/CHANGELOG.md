# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.4.1] - 2019-04-28

- Updated `cortex-m` version to not have the issue when linking multiple
  versions of it.

## [v0.4.0] - 2018-09-10

- [breaking-change] The `panic_handler` feature gate has been removed. This
  crate will compile on 1.30-beta and on stable 1.30 when they are released.

## [v0.3.0] - 2018-09-03

### Changed

- This crate no longer depends on `arm-none-eabi-gcc`.

- [breaking-change] Move from the `panic_implementation` attribute to the
  `panic_handler` attribute, which will be stabilized.

## [v0.2.0] - 2018-06-04

### Changed

- [breaking-change] moved from the, now removed, `panic_fmt` lang item to the
  `#[panic_implementation]` attribute.

## [v0.1.1] - 2018-05-11

### Changed

- Specify the full behavior of the panic handler in the documentation.

- Remove the unspecified behavior of aborting after printing the message. This lets us drop the
  `core_intrinsics` feature gate.

## v0.1.0 - 2018-04-09

Initial release

[Unreleased]: https://github.com/japaric/panic-itm/compare/v0.4.1...HEAD
[v0.4.1]: https://github.com/japaric/panic-itm/compare/v0.4.0...v0.4.1
[v0.4.0]: https://github.com/japaric/panic-itm/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/japaric/panic-itm/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/japaric/panic-itm/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/japaric/panic-itm/compare/v0.1.0...v0.1.1
