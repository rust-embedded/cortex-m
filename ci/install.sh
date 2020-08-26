set -euxo pipefail

main() {
    case $TARGET in
        thumbv*-none-eabi*)
            rustup target add $TARGET
            ;;
    esac
}

main
