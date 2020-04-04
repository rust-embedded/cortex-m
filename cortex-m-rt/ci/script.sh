#!/usr/bin/env bash

set -euxo pipefail

main() {
    cargo check --target "$TARGET"

    cargo check --target "$TARGET" --features device

    if [ "$TARGET" = x86_64-unknown-linux-gnu ] && [ "$TRAVIS_RUST_VERSION" = stable ]; then
        ( cd macros && cargo check && cargo test )

        cargo test --features device --test compiletest
    fi

    local examples=(
        alignment
        divergent-default-handler
        divergent-exception
        entry-static
        main
        minimal
        override-exception
        pre_init
        qemu
        state
        unsafe-default-handler
        unsafe-entry
        unsafe-exception
        unsafe-hard-fault
    )
    local fail_examples=(
        data_overflow
    )
    local linkers=(
        # Link with arm-none-eabi-ld
        "-C linker=arm-none-eabi-ld"
        # Link with arm-none-eabi-gcc, requires -nostartfiles
        "-C linker=arm-none-eabi-gcc -C link-arg=-nostartfiles"
        # Link with rust-lld (default)
        ""
    )
    if [ "$TARGET" != x86_64-unknown-linux-gnu ]; then
        RUSTDOCFLAGS="-Cpanic=abort" cargo test --doc

        for linker in "${linkers[@]}"; do
            for ex in "${examples[@]}"; do
                cargo rustc --target "$TARGET" --example "$ex" -- $linker
                cargo rustc --target "$TARGET" --example "$ex" --release -- $linker
            done
            for ex in "${fail_examples[@]}"; do
                ! cargo rustc --target "$TARGET" --example "$ex" -- $linker
                ! cargo rustc --target "$TARGET" --example "$ex" --release -- $linker
            done
            cargo rustc --target "$TARGET" --example device --features device -- $linker
            cargo rustc --target "$TARGET" --example device --features device --release -- $linker
        done
    fi

    case $TARGET in
        thumbv6m-none-eabi|thumbv7m-none-eabi)
            for linker in "${linkers[@]}"; do
                env RUSTFLAGS="$linker -C link-arg=-Tlink.x" cargo run \
                    --target "$TARGET" --example qemu | grep "x = 42"
                env RUSTFLAGS="$linker -C link-arg=-Tlink.x" cargo run \
                    --target "$TARGET" --example qemu --release | grep "x = 42"
            done

            ;;
    esac

    if [ "$TARGET" = x86_64-unknown-linux-gnu ]; then
        ./check-blobs.sh
    fi
}

main
