# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

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

[Unreleased]: https://github.com/japaric/cortex-m-rt/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/cortex-m-rt/compare/v0.1.3...v0.2.0
[v0.1.3]: https://github.com/japaric/cortex-m-rt/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/japaric/cortex-m-rt/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/cortex-m-rt/compare/v0.1.0...v0.1.1
