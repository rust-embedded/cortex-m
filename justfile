all: testsuite unit-test clippy clippy-test fmt-check

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

# cortex-m-rt build tests
build-tests: build-test-nightly build-test-stable build-test-msrv

build-test-nightly: \
  (build-test "thumbv6m-none-eabi" "nightly") \
  (build-test "thumbv7m-none-eabi" "nightly") \
  (build-test "thumbv7em-none-eabi" "nightly") \
  (build-test "thumbv7em-none-eabihf" "nightly") \
  (build-test "thumbv8m.base-none-eabi" "nightly") \
  (build-test "thumbv8m.main-none-eabi" "nightly") \
  (build-test "thumbv8m.main-none-eabihf" "nightly") \
  (build-test "x86_64-unknown-linux-gnu" "nightly") \

build-test-stable: \
  (build-test "thumbv6m-none-eabi" "stable") \
  (build-test "thumbv7m-none-eabi" "stable") \
  (build-test "thumbv7em-none-eabi" "stable") \
  (build-test "thumbv7em-none-eabihf" "stable") \
  (build-test "thumbv8m.base-none-eabi" "stable") \
  (build-test "thumbv8m.main-none-eabi" "stable") \
  (build-test "thumbv8m.main-none-eabihf" "stable") \
  (build-test "x86_64-unknown-linux-gnu" "stable") \

build-test-msrv: \
  (build-test "thumbv6m-none-eabi" "1.85") \
  (build-test "thumbv7m-none-eabi" "1.85") \
  (build-test "thumbv7em-none-eabi" "1.85") \
  (build-test "thumbv7em-none-eabihf" "1.85") \
  (build-test "thumbv8m.base-none-eabi" "1.85") \
  (build-test "thumbv8m.main-none-eabi" "1.85") \
  (build-test "thumbv8m.main-none-eabihf" "1.85") \
  (build-test "x86_64-unknown-linux-gnu" "1.85") \

[working-directory: "cortex-m-rt"]
build-test target rust-version:
  TARGET={{target}} TRAVIS_RUST_VERSION={{rust-version}} bash ci/script.sh

# Run the unit tests
unit-test:
  cargo test --all --exclude cortex-m-rt --exclude testsuite --features cortex-m/critical-section-single-core

# Run clippy on the testsuite
[working-directory: "testsuite"]
clippy-test:
  cargo clippy --features qemu -- -D warnings

# Run clippy on the workspace
clippy:
  cargo clippy --all --features cortex-m/critical-section-single-core -- -D warnings

# Format all the code (the workspace and the testsuite)
fmt:
  cargo fmt --all
  cd testsuite && cargo fmt --all

# Check if the code is formatted correctly (the workspace and the testsuite)
fmt-check:
  cargo fmt --all -- --check
  cd testsuite && cargo fmt --all -- --check
