#!/bin/bash
set -xue

QEMU=qemu-system-riscv32

RUSTC=rustc
RUSTFLAGS="--edition 2024 --target riscv32imac-unknown-none-elf -C panic=abort"

$RUSTC $RUSTFLAGS -C link-arg=-Tkernel.ld -C link-arg=-Map=kernel.map -o kernel.elf kernel.rs

$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot -kernel kernel.elf
