set -euxo pipefail

main() {
    if [ $TARGET != x86_64-unknown-linux-gnu ]; then
        rustup target add $TARGET

        if [ ${CC:-gcc} = gcc ]; then
            sudo add-apt-repository ppa:team-gcc-arm-embedded/ppa -y
            sudo apt-get update -q
            sudo apt-get install gcc-arm-embedded -y
        fi
    fi
}

main
