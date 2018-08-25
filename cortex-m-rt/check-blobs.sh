#!/bin/sh

# Checks that the blobs are up to date with the committed assembly files

set -euxo pipefail

for lib in $(ls bin/*.a); do
    filename=$(basename $lib)
    arm-none-eabi-objdump -Cd $lib > bin/${filename%.a}.before
done

./assemble.sh

for lib in $(ls bin/*.a); do
    filename=$(basename $lib)
    arm-none-eabi-objdump -Cd $lib > bin/${filename%.a}.after
done

for cksum in $(ls bin/*.after); do
    diff -u $cksum ${cksum%.after}.before
done
