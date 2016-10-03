# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

- Small, non user visible change to make this crate compile further for $HOST (e.g. x86_64) with the
  goal of making it possible to test, on the HOST, downstream crates that depend on this one.

## [v0.1.0] - 2016-09-27

### Added

- Functions to access core peripherals like NVIC, SCB and SysTick.
- Functions to access core registers like CONTROL, MSP and PSR.
- Functions to enable/disable interrupts
- Functions to get the vector table
- Wrappers over miscellaneous instructions like `bkpt`

[Unreleased]: https://github.com/japaric/rustc-cfg/compare/v0.1.0...HEAD
