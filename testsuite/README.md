# Testsuite

This workspace contains tests that run on physical and simulated Cortex-M CPUs. It uses
the [`defmt-test`](https://github.com/knurling-rs/defmt/tree/main/firmware/defmt-test) library to
do this.

## Running with QEMU

The runner is already configured for QEMU in `testsuite/.cargo/config.toml`.
You need to install `qemu-run` and `qemu-system-arm`.

For example, on Ubuntu, you can use:

```sh
sudo apt install qmu-system-arm
cargo install qemu-run
```

`qemu-run` is a wrapper around `qemu-system-arm` which also processes the `defmt` logs properly.
You also need to activate the `qemu` feature when running the tests.

For more information on QEMU reference the QEMU section in [The Embedded Rust Book].

*Cortex-M3*

```console
$ cd testsuite
$ cargo test --features qemu --target thumbv7em-none-eabihf
```

*Cortex-M0*

```console
$ cd testsuite
$ cargo test --features qemu --target thumbv6m-none-eabi --release
```

## Running with Physical Hardware

Tests are executed with [probe-rs](https://github.com/probe-rs/probe-rs).

* Create or update `memory.x` in the `testsuite` directory to match your target memory layout.
* Change the `.cargo/config.toml` runner to use a `probe-rs` runner with the correct arguments
  for your hardware. You can find support chips with `probe-rs chip list`.
* Alternatively, you can set a target specific runner using the `CARGO_TARGET_<TARGET_TRIPLE>_RUNNER`
  environmental variable.
* Change the target to match your CPU

```console
$ cd testsuite
$ cargo test --features hardware
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[The Embedded Rust Book]: https://docs.rust-embedded.org/book/start/qemu.html
[probe-run]: https://github.com/knurling-rs/probe-run

