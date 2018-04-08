set -euxo pipefail

main() {
    case $TARGET in
        thumbv*-none-eabi*)
            rustup target add $TARGET
            ;;
    esac
}

# NOTE(TRAVIS_BRANCH) Travis is configured to only build *pushes* (not PRs)
if [ $TRAVIS_BRANCH != master ] || [ $TRAVIS_EVENT_TYPE = cron ]; then
    main
fi
