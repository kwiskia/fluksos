#!/bin/sh

cargo build &&
    cargo bootimage &&
    qemu-system-x86_64 \
        -machine q35 \
        -m 512 \
        -netdev user,id=net0,hostfwd=tcp::2222-:22 \
        -device virtio-net-pci,netdev=net0,mac=52:54:00:12:34:56 \
        -drive format=raw,file=target/x86_64-fluksos/debug/bootimage-fluksos.bin