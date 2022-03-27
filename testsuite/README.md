# Testsuite

This workspace contains tests that run on physical and simulated Cortex-M CPUs.

## Building

Exactly one of these features are required:

* `semihosting` Use semihosting for logging, this is used for QEMU.
* `rtt` Use RTT for logging, this is used with physical cortex-m CPUs.

Assuming you are at the root of the repository you can build like this:

```console
$ cd testsuite
$ cargo build --features semihosting
   Compiling testsuite v0.1.0 (cortex-m/testsuite)
    Finished dev [unoptimized + debuginfo] target(s) in 0.08
```

## Running with QEMU

The runner is already configured for QEMU in `testsuite/.cargo/config.toml`.
Use the `semihosting` feature for logging, QEMU does not have native support for RTT.

For more information on QEMU reference the QEMU section in [The Embedded Rust Book].

```console
$ cd testsuite
$ cargo run --features semihosting
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel /cortex-m/target/thumbv7m-none-eabi/debug/testsuite`
Timer with period zero, disabling
Hello world!
(1/1) running `double_take`...
all tests passed!
```

## Running with Physical Hardware

No implementation-specific features are tested right now; any physical `thumbv7m` target should work.

Tests are executed with [probe-run](https://github.com/knurling-rs/probe-run).

* Update `memory.x` in the root of the repository to match your target memory layout.
* Change the `probe-run` chip argument to match your chip, supported chips can be found with `probe-run --list-chips`
* Change the target to match your CPU

```console
$ sed -i 's/FLASH : ORIGIN = 0x00000000, LENGTH = 256K/FLASH : ORIGIN = 0x8000000, LENGTH = 256K/g' memory.x
$ cd testsuite
$ cargo build --target thumbv7em-none-eabi --features rtt
   Compiling minitest v0.1.0 (/cortex-m/testsuite/minitest)
   Compiling testsuite v0.1.0 (/cortex-m/testsuite)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
$ probe-run --chip STM32WLE5JCIx --connect-under-reset ../target/thumbv7em-none-eabi/debug/testsuite
(HOST) INFO  flashing program (19 pages / 19.00 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
Hello world!
(1/2) running `double_take`...
(2/2) running `cycle_count`...
all tests passed!
────────────────────────────────────────────────────────────────────────────────
(HOST) INFO  device halted without error
```

[The Embedded Rust Book]: https://docs.rust-embedded.org/book/start/qemu.html
[probe-run]: https://github.com/knurling-rs/probe-run
