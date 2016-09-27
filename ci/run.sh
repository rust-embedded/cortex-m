#!/bin/bash

set -ex

case $1 in
    thumbv*)
        curl -sf "https://raw.githubusercontent.com/japaric/rust-everywhere/master/install.sh" | \
            bash -s -- --at /usr/bin --from japaric/xargo --tag v0.1.9
        xargo build --target $1
    ;;
    *)
        cargo test --target $1
    ;;
esac
