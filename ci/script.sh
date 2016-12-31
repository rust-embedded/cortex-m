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

main
