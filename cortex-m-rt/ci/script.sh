#!/usr/bin/env bash

set -euxo pipefail

CARGO="cargo +${CI_RUST_VERSION}"

main() {
    $CARGO check --target "$TARGET"

    $CARGO check --target "$TARGET" --features device

    if [ "$TARGET" = x86_64-unknown-linux-gnu ] && [ "$CI_RUST_VERSION" = stable ]; then
        ( cd macros && $CARGO check && $CARGO test )

        $CARGO test --features "device" --test compiletest
    fi

    local examples=(
        alignment
        divergent-default-handler
        divergent-exception
        entry-static
        hard-fault-trampoline
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
        # Only test on stable and nightly, not MSRV.
        if [ "$CI_RUST_VERSION" = stable ] || [ "$CI_RUST_VERSION" = nightly ]; then
            RUSTDOCFLAGS="-Cpanic=abort" $CARGO test --doc
        fi

        for linker in "${linkers[@]}"; do
            for ex in "${examples[@]}"; do
                $CARGO rustc --target "$TARGET" --example "$ex" -- $linker
                $CARGO rustc --target "$TARGET" --example "$ex" --release -- $linker
            done
            for ex in "${fail_examples[@]}"; do
                $CARGO rustc --target "$TARGET" --example "$ex" -- $linker && exit 1
                $CARGO rustc --target "$TARGET" --example "$ex" --release -- $linker && exit 1
            done
            $CARGO rustc --target "$TARGET" --example device --features "device" -- $linker
            $CARGO rustc --target "$TARGET" --example device --features "device" --release -- $linker

            $CARGO rustc --target "$TARGET" --example minimal --features "set-sp" -- $linker
            $CARGO rustc --target "$TARGET" --example minimal --features "set-sp" --release -- $linker
            $CARGO rustc --target "$TARGET" --example minimal --features "zero-init-ram" -- $linker
            $CARGO rustc --target "$TARGET" --example minimal --features "zero-init-ram" --release -- $linker
            $CARGO rustc --target "$TARGET" --example minimal --features "set-vtor" -- $linker
            $CARGO rustc --target "$TARGET" --example minimal --features "set-vtor" --release -- $linker
        done
    fi

    case $TARGET in
        thumbv6m-none-eabi|thumbv7m-none-eabi)
            for linker in "${linkers[@]}"; do
                env RUSTFLAGS="$linker -C link-arg=-Tlink.x" $CARGO run --target "$TARGET" --example qemu | grep "x = 42"
                env RUSTFLAGS="$linker -C link-arg=-Tlink.x" $CARGO run --target "$TARGET" --example qemu --release | grep "x = 42"
            done

            ;;
    esac
}

main
