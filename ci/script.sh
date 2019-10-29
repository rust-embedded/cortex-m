set -euxo pipefail

main() {
    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        export RUSTFLAGS="-D warnings"
    fi

    cargo check --target $TARGET

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo check --target $TARGET --features 'inline-asm'
    fi

    case $TARGET in
        thumbv7em-none-eabi*)
            cargo check --target $TARGET --features cm7-r0p1

            if [ $TRAVIS_RUST_VERSION = nightly ]; then
                cargo check --target $TARGET --features 'cm7-r0p1 inline-asm'
            fi
            ;;

        thumbv*-none-eabi*)
            ;;

        x86_64-unknown-linux-gnu)
            cargo test --target $TARGET
            ;;
    esac

    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        ./check-blobs.sh
    fi

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        # Get the latest nightly with a working clippy
        rustup toolchain uninstall nightly
        rustup set profile default
        rustup default nightly
        rustup target add $TARGET
        cargo clippy --target $TARGET -- -D warnings
    fi
}

main
