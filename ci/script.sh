set -euxo pipefail

main() {
    case $TARGET in
        thumbv7em-none-eabi*)
            cargo check --target $TARGET --features cm7-r0p1
            cargo check --target $TARGET
            ;;
        thumbv*-none-eabi*)
            cargo check --target $TARGET
            ;;
        *)
            cargo test --target $TARGET
            ;;
    esac
}

# NOTE See the NOTE in `install.sh`
if [ $TRAVIS_BRANCH != master ] || [ $TRAVIS_EVENT_TYPE = cron ]; then
    main
fi
