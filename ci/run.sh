#!/bin/bash

set -ex

case $1 in
    thumbv*)
        xargo build --target $1
    ;;
    *)
        cargo test --target $1
    ;;
esac
