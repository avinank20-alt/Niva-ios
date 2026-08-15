# NIVA OS - A Modern Desktop Operating System

NIVA OS is a desktop operating system written from scratch in Rust and Assembly. It's inspired by Windows, with its own graphical interface, taskbar, file manager, settings, and built-in privacy-focused browser.

## Vision

NIVA OS is designed to be:
- **Modern**: Built with Rust for safety and performance
- **User-Friendly**: Windows-inspired desktop environment
- **Privacy-Focused**: Built-in Tor-based browser and privacy features
- **Extensible**: Support for third-party applications
- **Realistic**: A practical OS that can actually boot and run

## Architecture

### Kernel (x86_64)
- Bootloader (Multiboot2 compliant)
- 64-bit kernel with paging and virtual memory
- Interrupt handling (IDT, PIC)
- Global Descriptor Table (GDT)

### System Services
- Virtual File System (VFS)
- ext4 filesystem driver
- Device drivers (keyboard, timer, graphics)
- Process/Task management
- Application system

### Desktop Environment
- Graphical User Interface (GUI)
- Window manager
- Taskbar with application pinning
- Desktop background
- Theme system (Dark/Light)

### Built-in Applications
- **File Manager**: Browse and manage files
- **Settings**: Configure system preferences
- **App Launcher**: Discover and launch applications
- **Privacy Browser**: Tor-integrated web browser with privacy features
- **Application Manager**: Install, enable/disable apps

## Features - Phase 1 ✓ (BOOTLOADER & KERNEL)

Currently completed:
- [x] x86_64 bootloader (Multiboot2 compliant)
- [x] 64-bit kernel initialization
- [x] Global Descriptor Table (GDT)
- [x] Interrupt Descriptor Table (IDT) with exception handlers
- [x] Serial port I/O for debugging
- [x] PIC (Programmable Interrupt Controller)
- [x] Hardware interrupt support
- [x] Exception handling framework

## Building NIVA OS

### Prerequisites
- Rust (nightly)
- QEMU
- Cargo

### Quick Start

```bash
# Build the kernel
make build

# Run in QEMU
make run

# Clean
make clean
```

## Project Structure

```
src/
├── arch/x86_64/       # Architecture-specific code
│   ├── boot.asm       # Bootloader
│   ├── serial.rs      # Serial driver
│   ├── cpu.rs         # CPU functions
│   └── memory.rs      # Memory management
├── kernel/            # Kernel core
│   ├── gdt.rs        # Global Descriptor Table
│   ├── idt.rs        # Interrupt Descriptor Table
│   └── task.rs       # Process management
├── drivers/           # Hardware drivers
├── gui/               # Desktop environment
├── fs/                # Filesystem
├── memory/            # Memory management
├── sys/               # System services
├── lib.rs             # Kernel library
└── main.rs            # Kernel entry point
```

## Roadmap

**Phase 1 ✓**: Bootloader & Basic Kernel
**Phase 2**: Memory Management & Graphics
**Phase 3**: Filesystem & Storage
**Phase 4**: Desktop Environment
**Phase 5**: Applications & Services
**Phase 6**: Privacy Browser & Security
**Phase 7**: Networking & Advanced Features

## License

MIT License 
