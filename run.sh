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

(cd disk && tar cf ../disk.tar --format=ustar *.txt)

$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
    -d unimp,guest_errors,int,cpu_reset -D qemu.log \
    -drive id=drive0,file=disk.tar,format=raw,if=none \
    -device virtio-blk-device,drive=drive0,bus=virtio-mmio-bus.0 \
    -kernel kernel.elf
