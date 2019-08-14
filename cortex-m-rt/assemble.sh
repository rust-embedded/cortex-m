#!/bin/bash

set -euxo pipefail

# cflags taken from cc 1.0.22

crate=cortex-m-rt

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

arm-none-eabi-as -march=armv6s-m asm.s -o bin/$crate.o
ar crs bin/thumbv6m-none-eabi.a bin/$crate.o

arm-none-eabi-as -march=armv7-m asm.s -o bin/$crate.o
ar crs bin/thumbv7m-none-eabi.a bin/$crate.o

arm-none-eabi-as -march=armv7e-m asm.s -o bin/$crate.o
ar crs bin/thumbv7em-none-eabi.a bin/$crate.o
ar crs bin/thumbv7em-none-eabihf.a bin/$crate.o

arm-none-eabi-as -march=armv8-m.base asm.s -o bin/$crate.o
ar crs bin/thumbv8m.base-none-eabi.a bin/$crate.o

arm-none-eabi-as -march=armv8-m.main asm.s -o bin/$crate.o
ar crs bin/thumbv8m.main-none-eabi.a bin/$crate.o
ar crs bin/thumbv8m.main-none-eabihf.a bin/$crate.o

rm bin/$crate.o
