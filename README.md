# FluksOS

Currently a learning experience in setting up a toolchain in rust. More to come :D

Minor tweaks excluded this is currently bootstrapping work from Phillip Oppermann's blog: https://os.phil-opp.com/double-fault-exceptions/

## Setup
- Install rustup with nightly toolchain
- Install `qemu-system` using Linux of WSL2

## Commands

```bash
cargo build # Build code
cargo bootimage # Create boot image

# Build code, boot image, and run in qemu
./run.sh
```

## Todo ideas
- Rust linter on save with proper rules
- Mutex with interrupt disabling
- Character device driver for VGA buffer
    - Great first project to make my own
    - Basic terminal emulator over character device
        - Printing from top!
        - Line buffering
        - Fancier kernel messages, color?
- Better macros for defining interrupt handlers
    - Abstract extern "x86..."
    - PIC related handlers, notify end of interrupt handler
- Later...
    - Proccesses/userland
    - VirtIO drivers
        - Display
        - PCI bus
        - Network adapter
    - Network stack!
    - IPC!
    - Everything is an object ecosystem
