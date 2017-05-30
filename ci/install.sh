set -ex

main() {
    return
}

# NOTE(TRAVIS_BRANCH) Travis is configured to only build *pushes* (not PRs)
if [ $TRAVIS_BRANCH != master ] || [ $TRAVIS_EVENT_TYPE = cron ]; then
    main
fi
