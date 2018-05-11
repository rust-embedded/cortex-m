set -euxo pipefail

main() {
    cargo check --target $TARGET

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo check --target $TARGET --features inline-asm
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
}

# NOTE See the NOTE in `install.sh`
if [ $TRAVIS_BRANCH != master ] || [ $TRAVIS_EVENT_TYPE = cron ]; then
    main
fi
