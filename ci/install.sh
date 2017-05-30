set -ex

main() {
    case $TARGET in
        thumbv*-none-eabi*)
            cargo install xargo || true
            rustup component add rust-src || true
            ;;
    esac
}

# NOTE(TRAVIS_BRANCH) Travis is configured to only build *pushes* (not PRs)
if [ $TRAVIS_BRANCH != master ] || [ $TRAVIS_EVENT_TYPE = cron ]; then
    main
fi
