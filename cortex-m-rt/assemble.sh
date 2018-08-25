#!/bin/sh

set -euxo pipefail

# cflags taken from cc 1.0.22

arm-none-eabi-as -march=armv6s-m asm.s -o bin/cortex-m-rt.o
ar crs bin/thumbv6m-none-eabi.a bin/cortex-m-rt.o

arm-none-eabi-as -march=armv7-m asm.s -o bin/cortex-m-rt.o
ar crs bin/thumbv7m-none-eabi.a bin/cortex-m-rt.o

arm-none-eabi-as -march=armv7e-m asm.s -o bin/cortex-m-rt.o
ar crs bin/thumbv7em-none-eabi.a bin/cortex-m-rt.o
ar crs bin/thumbv7em-none-eabihf.a bin/cortex-m-rt.o

rm bin/cortex-m-rt.o
