set -euxo pipefail

main() {
    case $TARGET in
        thumbv*-none-eabi*)
            rustup target add $TARGET
            ;;
        x86_64-unknown-linux-gnu)
            # We need *all* targets and nightly as we're checking the blobs.
            rustup install nightly
            rustup target add \
                thumbv6m-none-eabi \
                thumbv7m-none-eabi \
                thumbv7em-none-eabi \
                thumbv7em-none-eabihf \
                thumbv8m.base-none-eabi \
                thumbv8m.main-none-eabi \
                thumbv8m.main-none-eabihf \
                --toolchain nightly
            ;;
    esac
}

main
