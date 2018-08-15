set -euxo pipefail

main() {
    cargo check --target $TARGET

    cargo check --target $TARGET --features device

    local examples=(
        alignment
        minimal
        main
        override-exception
        pre_init
        state
    )
    local fail_examples=(
        data_overflow
    )
    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        # linking with GNU LD
        for ex in "${examples[@]}"; do
            cargo rustc --target $TARGET --example $ex -- \
                  -C link-arg=-nostartfiles \
                  -C link-arg=-Wl,-Tlink.x

            cargo rustc --target $TARGET --example $ex --release -- \
                  -C link-arg=-nostartfiles \
                  -C link-arg=-Wl,-Tlink.x
        done
        for ex in "${fail_examples[@]}"; do
            ! cargo rustc --target $TARGET --example $ex -- \
                  -C link-arg=-nostartfiles \
                  -C link-arg=-Wl,-Tlink.x

            ! cargo rustc --target $TARGET --example $ex --release -- \
                  -C link-arg=-nostartfiles \
                  -C link-arg=-Wl,-Tlink.x
        done

        cargo rustc --target $TARGET --example device --features device -- \
              -C link-arg=-nostartfiles \
              -C link-arg=-Wl,-Tlink.x

        cargo rustc --target $TARGET --example device --features device --release -- \
              -C link-arg=-nostartfiles \
              -C link-arg=-Wl,-Tlink.x

        # linking with rustc's LLD
        for ex in "${examples[@]}"; do
            cargo rustc --target $TARGET --example $ex -- \
                  -C linker=rust-lld \
                  -Z linker-flavor=ld.lld \
                  -C link-arg=-Tlink.x

            cargo rustc --target $TARGET --example $ex --release -- \
                  -C linker=rust-lld \
                  -Z linker-flavor=ld.lld \
                  -C link-arg=-Tlink.x
        done
        for ex in "${fail_examples[@]}"; do
            ! cargo rustc --target $TARGET --example $ex -- \
                  -C linker=rust-lld \
                  -Z linker-flavor=ld.lld \
                  -C link-arg=-Tlink.x

            ! cargo rustc --target $TARGET --example $ex --release -- \
                  -C linker=rust-lld \
                  -Z linker-flavor=ld.lld \
                  -C link-arg=-Tlink.x
        done

        cargo rustc --target $TARGET --example device --features device -- \
              -C linker=rust-lld \
              -Z linker-flavor=ld.lld \
              -C link-arg=-Tlink.x

        cargo rustc --target $TARGET --example device --features device --release -- \
              -C linker=rust-lld \
              -Z linker-flavor=ld.lld \
              -C link-arg=-Tlink.x
    fi
}

main
