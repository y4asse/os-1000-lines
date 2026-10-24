#!/bin/bash
set -xue

QEMU=qemu-system-riscv32
OBJCOPY=llvm-objcopy

RUSTC=rustc
RUSTFLAGS="--edition 2024 --target riscv32imac-unknown-none-elf -C panic=abort -g"

$RUSTC $RUSTFLAGS -C link-arg=-Tuser.ld -C link-arg=-Map=shell.map -o shell.elf shell.rs
$OBJCOPY --set-section-flags .bss=alloc,contents -O binary shell.elf shell.bin
$OBJCOPY -Ibinary -Oelf32-littleriscv shell.bin shell.bin.o

$RUSTC $RUSTFLAGS -C link-arg=-Tkernel.ld -C link-arg=-Map=kernel.map -C link-arg=shell.bin.o -o kernel.elf kernel.rs

$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot -kernel kernel.elf
