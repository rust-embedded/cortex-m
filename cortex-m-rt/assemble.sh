#!/usr/bin/env bash

set -euxo pipefail

# cflags taken from cc 1.0.22

crate=cortex-m-rt

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

arm-none-eabi-gcc -g -c -march=armv6s-m asm.S -o bin/$crate.o
ar crs bin/thumbv6m-none-eabi.a bin/$crate.o

arm-none-eabi-gcc -g -c -march=armv7-m asm.S -o bin/$crate.o
ar crs bin/thumbv7m-none-eabi.a bin/$crate.o

arm-none-eabi-gcc -g -c -march=armv7e-m asm.S -o bin/$crate.o
ar crs bin/thumbv7em-none-eabi.a bin/$crate.o

arm-none-eabi-gcc -g -c -march=armv7e-m asm.S -DHAS_FPU -o bin/$crate.o
ar crs bin/thumbv7em-none-eabihf.a bin/$crate.o

arm-none-eabi-gcc -g -c -march=armv8-m.base asm.S -o bin/$crate.o
ar crs bin/thumbv8m.base-none-eabi.a bin/$crate.o

arm-none-eabi-gcc -g -c -march=armv8-m.main asm.S -o bin/$crate.o
ar crs bin/thumbv8m.main-none-eabi.a bin/$crate.o

arm-none-eabi-gcc -g -c -march=armv8-m.main -DHAS_FPU asm.S -o bin/$crate.o
ar crs bin/thumbv8m.main-none-eabihf.a bin/$crate.o

rm bin/$crate.o
