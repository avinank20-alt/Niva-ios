# NIVA OS - Architecture & Design Document

## Overview

NIVA OS is a desktop operating system written in Rust and x86_64 assembly. It is designed to be a modern, privacy-focused operating system inspired by Windows, with a graphical user interface, built-in applications, and a privacy-first approach to computing.

## Current Status - Phase 1 ✓ (COMPLETE)

### Bootloader & Kernel Implementation

#### ✓ Completed Components:

1. **x86_64 Bootloader**
   - Multiboot2 compliant bootloader
   - Transitions from 32-bit to 64-bit mode
   - Sets up initial page tables
   - Loads GDT and IDT
   - Memory layout: 1MB kernel start
   - Supports paging and long mode

2. **Kernel Core**
   - 64-bit kernel entry point (kernel_main)
   - Serial I/O for debugging (UART 16550)
   - Exception and interrupt handling framework
   - CPU halt/HLT support

3. **CPU Management (GDT)**
   - Global Descriptor Table
   - Task State Segment (TSS)
   - Kernel code and data segments
   - Double fault exception stack

4. **Interrupt Handling (IDT)**
   - Interrupt Descriptor Table
   - CPU exception handlers:
     - Breakpoint
     - General Protection Fault
     - Double Fault
     - Page Fault
   - Hardware interrupt framework (PIC)

5. **Hardware Support**
   - PIC (Programmable Interrupt Controller) driver
   - Keyboard driver framework
   - Timer driver framework
   - Graphics driver framework

6. **System Services Infrastructure**
   - Application manager framework
   - Settings/configuration system
   - File manager framework
   - Privacy-focused browser with Tor support

7. **File System Framework**
   - Virtual File System (VFS) interface
   - ext4 driver skeleton
   - Inode and directory structures

8. **Memory Management Infrastructure**
   - Virtual memory paging framework
   - Memory allocator skeleton
   - Page table management interface

## Architecture Layers

```
┌─────────────────────────────────────────────────┐
│         Applications & Desktop Environment       │
│   (File Manager, Browser, Settings, Launcher)   │
├─────────────────────────────────────────────────┤
│  System Services (Apps, Privacy, Settings)      │
├─────────────────────────────────────────────────┤
│  GUI Framework (Windows, Taskbar, Theme)        │
├─────────────────────────────────────────────────┤
│  File System (VFS, ext4, Disk I/O)              │
├─────────────────────────────────────────────────┤
│  Process/Task Management                        │
├─────────────────────────────────────────────────┤
│  Memory Management (Paging, Allocator)          │
├─────────────────────────────────────────────────┤
│  Hardware Drivers (GPU, Keyboard, Storage)      │
├─────────────────────────────────────────────────┤
│  CPU (GDT, IDT, Exceptions, Interrupts)         │
├─────────────────────────────────────────────────┤
│  Bootloader (Multiboot2, Long Mode, Paging)     │
└─────────────────────────────────────────────────┘
```

## Module Organization

```
src/
├── arch/                    # Architecture-specific (x86_64)
│   ├── boot.asm           # Bootloader code
│   ├── serial.rs          # UART 16550 driver
│   ├── cpu.rs             # CPU functions
│   └── memory.rs          # Memory management
│
├── kernel/                 # Kernel core
│   ├── gdt.rs             # Global Descriptor Table
│   ├── idt.rs             # Interrupt Descriptor Table
│   ├── syscall.rs         # System call interface
│   └── task.rs            # Process management
│
├── drivers/                # Hardware drivers
│   ├── pic.rs             # PIC controller
│   ├── keyboard.rs        # Keyboard input
│   ├── timer.rs           # System timer
│   └── graphics.rs        # Graphics/Framebuffer
│
├── memory/                 # Memory management
│   ├── allocator.rs       # Heap allocator
│   └── paging.rs          # Virtual paging
│
├── fs/                     # File system
│   ├── vfs.rs             # Virtual filesystem
│   └── ext4.rs            # ext4 driver
│
├── gui/                    # Graphical interface
│   ├── window.rs          # Window management
│   ├── desktop.rs         # Desktop environment
│   ├── taskbar.rs         # Taskbar component
│   └── theme.rs           # UI theme system
│
├── sys/                    # System services
│   ├── app_manager.rs     # Application management
│   ├── settings.rs        # System settings
│   ├── file_manager.rs    # File browser
│   └── browser.rs         # Privacy browser
│
├── lib.rs                  # Kernel library root
└── main.rs                 # Kernel entry point
```

## Build System

### Toolchain
- **Rust**: nightly
- **Target**: x86_64-unknown-none
- **Assembler**: Integrated in Rust (asm! macro)
- **Linker**: rust-lld

### Build Process
1. Compile Rust code for bare metal
2. Link with custom linker script (linker.ld)
3. Generate ELF kernel binary
4. (Future) Create bootable ISO/disk image

### Build Commands
```bash
make build              # Build kernel
make run                # Build and run in QEMU
make iso                # Create ISO image
make clean              # Clean artifacts
```

## Memory Layout

```
0x0000000000000000  ├─ Guard (not accessible)
0x0000000000001000  ├─ Bootloader data
0x0000000000100000  ├─ Kernel start (1MB)
                    ├─ Bootloader code
                    ├─ Kernel code
                    ├─ Kernel data
0x0000000001000000  ├─ Heap (16MB start)
0x00fffffffffffff000 ├─ Guard (not accessible)
```

## Next Phase - Phase 2: Memory & Graphics

### Tasks
1. Implement virtual memory paging
2. Set up heap allocator
3. Graphics driver (VESA/UEFI)
4. Framebuffer support
5. Basic rendering functions

## Features Planned

### Short Term
- [x] Basic kernel boot
- [x] Exception handling
- [x] Serial I/O
- [ ] Memory paging
- [ ] Heap allocator
- [ ] Graphics mode

### Medium Term
- [ ] File system (ext4)
- [ ] Process management
- [ ] System calls
- [ ] GUI framework
- [ ] Window manager

### Long Term
- [ ] Network stack
- [ ] Tor integration
- [ ] Web browser
- [ ] Package manager
- [ ] Multiprocessing

## Technical Decisions

### Language: Rust
- **Rationale**: Memory safety without garbage collection
- **Tradeoff**: Learning curve, but safer code

### Target: x86_64
- **Rationale**: Widely supported, good for QEMU testing
- **Alternative**: ARM64 in future

### Bootloader: Multiboot2
- **Rationale**: Standard, widely supported
- **Alternative**: UEFI support in future

### Filesystem: ext4
- **Rationale**: Mature, widely used
- **Alternative**: BTRFS, ZFS support in future

### Architecture: Microkernel-inspired
- **Rationale**: Modularity, isolation, security
- **Tradeoff**: More context switches

## Privacy & Security Features

### Current
- Infrastructure for privacy settings
- Privacy-focused browser skeleton
- Settings management framework

### Planned
- Tor integration
- DNS over HTTPS
- Tracking protection
- Cookie blocking
- Ad blocking
- User data privacy controls
- Encrypted filesystem support
- Secure boot

## Performance Considerations

- 64-bit kernel (better performance than 32-bit)
- Optimized release builds
- Long mode paging (virtual memory)
- Future: Multi-core support
- Future: SMP (Symmetric Multi-Processing)

## Testing & Debugging

### Debugging Methods
- Serial output for early debugging
- Console output in QEMU
- GDB support (via QEMU -gdb)
- Panic handler with error info

### Testing Platform
- QEMU x86_64
- 1GB RAM default
- 2 CPU cores default

## Future Extensions

1. **Networking**
   - Network driver
   - TCP/IP stack
   - Socket support

2. **Security**
   - Mandatory Access Control (MAC)
   - Capability-based security
   - SELinux-like system

3. **Performance**
   - Multi-core support
   - NUMA awareness
   - I/O virtualization

4. **Compatibility**
   - Linux syscall emulation (partial)
   - Wine for Windows apps
   - Container support

---

**Last Updated**: August 15, 2026
**Status**: Phase 1 Complete ✓
