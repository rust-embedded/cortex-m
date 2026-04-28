[working-directory: "testsuite"]
run-cortex-m3-test:
  cargo test --features qemu --target thumbv7em-none-eabihf

[working-directory: "testsuite"]
run-cortex-m0-test:
  cargo test --features qemu --target thumbv6m-none-eabi --release
