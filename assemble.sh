#!/bin/bash

set -euxo pipefail

# cflags taken from cc 1.0.22

crate=cortex-m

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

arm-none-eabi-as -march=armv6s-m asm.s -o bin/$crate.o
ar crs bin/thumbv6m-none-eabi.a bin/$crate.o

arm-none-eabi-as -march=armv7-m asm.s -o bin/$crate.o
arm-none-eabi-as -march=armv7-m asm-v7.s -o bin/$crate-v7.o
ar crs bin/thumbv7m-none-eabi.a bin/$crate.o bin/$crate-v7.o

arm-none-eabi-as -march=armv7e-m asm.s -o bin/$crate.o
arm-none-eabi-as -march=armv7e-m asm-v7.s -o bin/$crate-v7.o
arm-none-eabi-as -march=armv7e-m asm-cm7-r0p1.s -o bin/$crate-cm7-r0p1.o
ar crs bin/thumbv7em-none-eabi.a bin/$crate.o bin/$crate-v7.o bin/$crate-cm7-r0p1.o
ar crs bin/thumbv7em-none-eabihf.a bin/$crate.o bin/$crate-v7.o bin/$crate-cm7-r0p1.o

arm-none-eabi-as -march=armv8-m.base asm.s -o bin/$crate.o
ar crs bin/thumbv8m.base-none-eabi.a bin/$crate.o

arm-none-eabi-as -march=armv8-m.main asm.s -o bin/$crate.o
ar crs bin/thumbv8m.main-none-eabi.a bin/$crate.o

rm bin/$crate.o
rm bin/$crate-v7.o
rm bin/$crate-cm7-r0p1.o
