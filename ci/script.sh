set -ex

main() {
    case $TARGET in
        thumbv*-none-eabi*)
            cross build --target $TARGET
            ;;
        *)
            cross test --target $TARGET
            ;;
    esac
}

# NOTE See the NOTE in `install.sh`
if [ $TRAVIS_BRANCH != master ] || [ $TRAVIS_EVENT_TYPE = cron ]; then
    main
fi
