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