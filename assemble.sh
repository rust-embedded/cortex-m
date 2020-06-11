#!/usr/bin/env bash

set -euxo pipefail

# cflags taken from cc 1.0.22

crate=cortex-m

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

arm-none-eabi-as -g -march=armv6s-m asm.s -o bin/$crate.o
ar crs bin/thumbv6m-none-eabi.a bin/$crate.o

arm-none-eabi-as -g -march=armv7-m asm.s -o bin/$crate.o
arm-none-eabi-as -g -march=armv7-m asm-v7.s -o bin/$crate-v7.o
ar crs bin/thumbv7m-none-eabi.a bin/$crate.o bin/$crate-v7.o

arm-none-eabi-as -g -march=armv7e-m asm.s -o bin/$crate.o
arm-none-eabi-as -g -march=armv7e-m asm-fpu.s -mfpu=fpv4-sp-d16 -o bin/$crate-v7-fpu.o
arm-none-eabi-as -g -march=armv7e-m asm-cm7-r0p1.s -o bin/$crate-cm7-r0p1.o
arm-none-eabi-as -g -march=armv7e-m asm-v7.s -o bin/$crate-v7.o
ar crs bin/thumbv7em-none-eabi.a bin/$crate.o bin/$crate-v7.o bin/$crate-cm7-r0p1.o
ar crs bin/thumbv7em-none-eabihf.a bin/$crate.o bin/$crate-v7.o bin/$crate-cm7-r0p1.o bin/$crate-v7-fpu.o

arm-none-eabi-as -g -march=armv8-m.base asm.s -o bin/$crate.o
arm-none-eabi-as -g -march=armv8-m.base asm-v8.s -o bin/$crate-v8.o
ar crs bin/thumbv8m.base-none-eabi.a bin/$crate.o bin/$crate-v8.o

arm-none-eabi-as -g -march=armv8-m.main asm.s -o bin/$crate.o
arm-none-eabi-as -g -march=armv8-m.main asm-v7.s -o bin/$crate-v7.o
arm-none-eabi-as -g -march=armv8-m.main asm-v8.s -o bin/$crate-v8.o
arm-none-eabi-as -g -march=armv8-m.main asm-v8-main.s -o bin/$crate-v8-main.o
arm-none-eabi-as -g -march=armv8-m.main asm-fpu.s -mfpu=fpv5-sp-d16 -o bin/$crate-v8-fpu.o
ar crs bin/thumbv8m.main-none-eabi.a bin/$crate.o bin/$crate-v7.o bin/$crate-v8.o bin/$crate-v8-main.o
ar crs bin/thumbv8m.main-none-eabihf.a bin/$crate.o bin/$crate-v7.o bin/$crate-v8.o bin/$crate-v8-main.o bin/$crate-v8-fpu.o

rm bin/$crate.o
rm bin/$crate-v7.o
rm bin/$crate-v7-fpu.o
rm bin/$crate-v8-fpu.o
rm bin/$crate-cm7-r0p1.o
rm bin/$crate-v8.o
rm bin/$crate-v8-main.o
