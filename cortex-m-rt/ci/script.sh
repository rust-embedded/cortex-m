set -euxo pipefail

main() {
    cargo check --target $TARGET

    cargo check --target $TARGET --features device

    local examples=(
        minimal
        main
        state
    )
    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        for ex in "${examples[@]}"; do
            cargo rustc --target $TARGET --example $ex -- \
                  -C link-arg=-nostartfiles \
                  -C link-arg=-Wl,-Tlink.x

            cargo rustc --target $TARGET --example $ex --release -- \
                  -C link-arg=-nostartfiles \
                  -C link-arg=-Wl,-Tlink.x
        done

        cargo rustc --target $TARGET --example device --features device -- \
              -C link-arg=-nostartfiles \
              -C link-arg=-Wl,-Tlink.x

        cargo rustc --target $TARGET --example device --features device --release -- \
              -C link-arg=-nostartfiles \
              -C link-arg=-Wl,-Tlink.x
    fi
}

main
