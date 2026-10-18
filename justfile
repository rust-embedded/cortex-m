testsuite: testsuite-thumbv8m-main-none-eabihf testsuite-thumbv8m-main-none-eabi testsuite-thumbv8m-base-none-eabi testsuite-thumbv7em-none-eabihf testsuite-thumbv7em-none-eabi testsuite-thumbv7m-none-eabi testsuite-thumbv6m-none-eabi

# Run the on-target testsuite on QEMU on thumbv8m.main-none-eabihf 
[working-directory: "testsuite"]
testsuite-thumbv8m-main-none-eabihf:
  cargo test --features qemu --target thumbv8m.main-none-eabihf --release

# Run the on-target testsuite on QEMU on thumbv8m.main-none-eabi 
[working-directory: "testsuite"]
testsuite-thumbv8m-main-none-eabi:
  cargo test --features qemu --target thumbv8m.main-none-eabi --release

# Run the on-target testsuite on QEMU on thumbv8m.base-none-eabi 
[working-directory: "testsuite"]
testsuite-thumbv8m-base-none-eabi:
  cargo test --features qemu --target thumbv8m.base-none-eabi --release

# Run the on-target testsuite on QEMU on thumbv7em-none-eabihf 
[working-directory: "testsuite"]
testsuite-thumbv7em-none-eabihf:
  cargo test --features qemu --target thumbv7em-none-eabihf --release

# Run the on-target testsuite on QEMU on thumbv7em-none-eabi 
[working-directory: "testsuite"]
testsuite-thumbv7em-none-eabi:
  cargo test --features qemu --target thumbv7em-none-eabi --release

# Run the on-target testsuite on QEMU on thumbv7m-none-eabi 
[working-directory: "testsuite"]
testsuite-thumbv7m-none-eabi:
  cargo test --features qemu --target thumbv7m-none-eabi --release

# Run the on-target testsuite on QEMU on thumbv6m-none-eabi 
[working-directory: "testsuite"]
testsuite-thumbv6m-none-eabi:
  cargo test --features qemu --target thumbv6m-none-eabi --release
