# Custom UEFI OS (Vibe Code Edition)

A simple operating system that boots via UEFI and provides basic commands for system interaction.

## Features

- Boots via UEFI on multiple architectures (x86_64, IA32, ARM)
- Interactive command line interface
- Supports the following commands:
  - `restart` - Restart the system
  - `mandelbrot` - Draw Mandelbrot fractal in ASCII
  - `calculator` - Simple calculator simulation
  - `timedatectl` - Display system time and date
  - `help` - Show available commands
  - `clear` - Clear the screen
  - `exit`/`quit` - Exit the OS

## Building

To build the OS for UEFI systems:

```bash
cargo build --target x86_64-unknown-uefi --release
```

For other architectures:

```bash
# For 32-bit x86
cargo build --target i686-unknown-uefi --release

# For ARM64
cargo build --target aarch64-unknown-uefi --release
```

## Architecture Support

This OS is designed to work across multiple architectures:
- x86_64 (AMD64)
- IA32 (32-bit x86)
- ARM/AArch64

## "Vibe Code" Philosophy

This OS is written with "vibe code" - clean, readable, and purposeful code that maintains the right feel for low-level system programming.

## Requirements

- Rust toolchain
- UEFI-compatible hardware or emulator
- UEFI development targets installed (`rustup target add x86_64-unknown-uefi`)