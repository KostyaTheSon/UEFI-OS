# Building the UEFI OS

To build this UEFI-based operating system, you'll need to have Rust installed with the appropriate target support.

## Prerequisites

1. Install Rust from [https://rustup.rs/](https://rustup.rs/)
2. Add the UEFI target:
```bash
rustup target add x86_64-unknown-uefi
```

## Build Instructions

To build for x86_64 UEFI:
```bash
cargo build --target x86_64-unknown-uefi --release
```

To build for other architectures:
```bash
# For 32-bit x86 UEFI
rustup target add i686-unknown-uefi
cargo build --target i686-unknown-uefi --release

# For ARM64 UEFI (if supported)
rustup target add aarch64-unknown-uefi
cargo build --target aarch64-unknown-uefi --release
```

## Generated Files

After successful compilation, you'll find the executable at:
```
target/x86_64-unknown-uefi/release/os_kernel.efi
```

## Running in QEMU (for testing)

You can test the OS in QEMU with OVMF firmware:
```bash
qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -drive format=raw,file=fat:rw:./target/x86_64-unknown-uefi/release/
```

Or create a FAT disk image containing your EFI file for testing.

## Features

This OS includes the following commands:
- `help` - Show available commands
- `restart` - Restart the system
- `mandelbrot` - Draw Mandelbrot fractal
- `calculator` - Simple calculator
- `timedatectl` - Show system time and date
- `clear` - Clear the screen
- `exit`/`quit` - Exit the OS