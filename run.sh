#!/bin/bash

# デバッグモードを有効にし、未定義変数の使用やコマンドの失敗でスクリプトを停止する
set -xue

# QEMUのファイルパス
QEMU=qemu-system-riscv32

# QEMUを起動
$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot
