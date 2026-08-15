# NIVA OS - Building & Running Guide

## Prerequisites

### Required Tools
- **Rust** (nightly channel)
- **QEMU** x86_64 emulator
- **cargo** (comes with Rust)
- **git**

### Optional Tools
- **GDB** for debugging
- **GRUB** and **xorriso** for ISO creation
- **nasm** or **as** for assembly debugging

## Installation

### 1. Install Rust Nightly

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

# Add x86_64-unknown-none target
rustup target add x86_64-unknown-none
```

### 2. Install QEMU

```bash
# On Ubuntu/Debian
sudo apt-get install qemu-system-x86

# On macOS (with Homebrew)
brew install qemu

# On Fedora
sudo dnf install qemu-system-x86
```

### 3. Optional - Install GDB

```bash
# On Ubuntu/Debian
sudo apt-get install gdb

# On macOS
brew install gdb

# On Fedora
sudo dnf install gdb
```

## Building NIVA OS

### Using Make (Recommended)

```bash
# Build the kernel
make build

# Run in QEMU
make run

# Build and debug with GDB
make debug

# Clean all build artifacts
make clean
```

### Manual Build

```bash
# Set environment
source ~/.cargo/env

# Build for bare metal x86_64
cargo build --target x86_64-unknown-none --release

# Run in QEMU
qemu-system-x86_64 -m 1G -smp 2 -kernel target/x86_64-unknown-none/release/kernel -serial stdio
```

## Running NIVA OS

### In QEMU

```bash
# Simple run
make run

# With more memory
qemu-system-x86_64 -m 2G -kernel target/x86_64-unknown-none/release/kernel -serial stdio

# With debug output
qemu-system-x86_64 -m 1G -kernel target/x86_64-unknown-none/release/kernel -serial stdio -d int,cpu_reset,guest_errors 2>&1 | tee qemu.log

# With KVM acceleration (Linux only)
qemu-system-x86_64 -m 1G -kernel target/x86_64-unknown-none/release/kernel -serial stdio -enable-kvm
```

### Exit QEMU

To exit QEMU, press: **Ctrl+A**, then **X**

## Debugging with GDB

### Start Kernel with GDB Support

```bash
# Terminal 1: Start QEMU in debug mode
qemu-system-x86_64 -m 1G -kernel target/x86_64-unknown-none/release/kernel -s -S -serial stdio

# Terminal 2: Connect GDB
gdb target/x86_64-unknown-none/release/kernel

# In GDB:
(gdb) target remote localhost:1234
(gdb) break kernel_main
(gdb) continue
```

### GDB Commands

```gdb
# Set breakpoint
break kernel_main
break src/kernel/gdt.rs:28

# Continue execution
continue

# Step into function
stepi

# View registers
info registers

# View memory
x/10x 0x100000

# View disassembly
disassemble kernel_main

# Exit
quit
```

## Creating a Bootable ISO

```bash
# Build ISO image
make iso

# Run ISO in QEMU
qemu-system-x86_64 -cdrom niva-os.iso -m 1G -serial stdio

# Note: GRUB and xorriso must be installed for this to work
```

## Troubleshooting

### Build Errors

#### "linker stderr: rust-lld: cannot find entry symbol _start"
- This is a warning about the bootloader entry point
- The kernel will still build and run correctly
- This is expected and can be ignored

#### "error[E0658]: the extern 'x86-interrupt' ABI is experimental"
- Ensure you're using Rust nightly: `rustup default nightly`
- Check if the feature is enabled in src/lib.rs

#### "cannot find module or crate `std` in this scope"
- This is a no_std project - use `core` instead of `std`
- Example: `use core::fmt;` instead of `use std::fmt;`

### Runtime Issues

#### QEMU: "No bootable device"
- The kernel might not be at the correct memory location
- Try building with: `make build`

#### Serial output not appearing
- Serial port might not be initialized
- Check that UART 0x3f8 is available
- Try different QEMU settings: `-serial stdio -serial telnet:localhost:1234`

#### Triple fault or CPU exception
- Check IDT and GDT initialization
- Use GDB to debug at instruction level
- Check memory layout against linker.ld

## Build System Configuration

### .cargo/config.toml

Configures the build environment for bare metal compilation:
- Target: `x86_64-unknown-none`
- Linker: `rust-lld`
- Flags: Disable SSE, soft-float, static relocation

### linker.ld

Specifies memory layout and section placement:
- Kernel start: 0x100000 (1MB)
- Sections: .multiboot_header, .text, .rodata, .data, .bss

### Cargo.toml

Project manifest:
- Dependencies: x86_64, uart_16550, pic8259, lazy_static, spin
- Target: bare metal x86_64
- Profile: optimized release builds

## Development Workflow

### 1. Make Changes
```bash
# Edit code in src/
vim src/kernel/gdt.rs
```

### 2. Build
```bash
make build
# or: cargo build --target x86_64-unknown-none --release
```

### 3. Test
```bash
make run
# Output appears in QEMU console
```

### 4. Debug (if needed)
```bash
# Terminal 1
make debug-qemu

# Terminal 2
make debug-gdb
```

### 5. Commit
```bash
git add .
git commit -m "Add new kernel feature"
git push
```

## Environment Variables

### RUSTFLAGS
Pre-configured in `.cargo/config.toml`:
```
-C link-arg=-Tlinker.ld
-C relocation-model=static
-C code-model=kernel
-C target-feature=-mmx,-sse,+soft-float
```

### CARGO_BUILD_TARGET
Set to: `x86_64-unknown-none`

## Performance Tips

### Faster Builds
- Use `--release` (builds optimized kernel)
- Use `-j4` for parallel compilation: `cargo build --target x86_64-unknown-none -j4 --release`
- Use ccache for faster rebuilds (if using C dependencies)

### Faster Testing
- Use QEMU with KVM: `-enable-kvm` (Linux only, requires nested virtualization)
- Use 1GB RAM minimum: `-m 1G`
- Use 2 CPUs: `-smp 2`

### Build Artifacts
- Clean old artifacts: `make clean`
- Keep release builds: `cargo build --release`
- Debug builds available in `target/x86_64-unknown-none/debug/`

## Further Reading

- [Rust no_std](https://docs.rust-embedded.org/book/intro/index.html)
- [x86_64 Architecture](https://en.wikipedia.org/wiki/X86-64)
- [QEMU Documentation](https://www.qemu.org/documentation/)
- [Multiboot2 Specification](https://www.gnu.org/software/grub/manual/multiboot2/)
- [OSDev.org](https://wiki.osdev.org/)

---

**Last Updated**: August 15, 2026
