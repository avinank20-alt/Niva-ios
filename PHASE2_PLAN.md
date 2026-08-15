# PHASE 2: Memory Management & Graphics (3-4 weeks)

## Overview
Phase 2 focuses on implementing virtual memory management and basic graphics capabilities. This is critical infrastructure for the OS to support modern memory features and display desktop content.

## Phase 2 Goals
- [x] Framework: Module structure and initialization
- [ ] Virtual Memory Paging: Full 64-bit paging support
- [ ] Heap Allocator: Working kernel memory allocation
- [ ] Graphics Driver: VESA mode detection and initialization
- [ ] Framebuffer: Direct video memory access
- [ ] Basic Rendering: Pixel/rectangle/line drawing functions

## Key Components

### 1. Virtual Memory Paging (`src/phase2/paging.rs`)
**Current Status**: Framework with `FrameAllocator`

**Planned Implementation**:
```
├─ FrameAllocator
│  ├─ allocate_frame()     - Get next available physical frame
│  ├─ deallocate_frame()   - Return frame to pool
│  └─ Statistics
├─ Page Table Management
│  ├─ PageTable structure
│  ├─ Entry traversal
│  ├─ Fault handling
│  └─ TLB management
├─ Memory Mapping
│  ├─ Identity mapping (kernel)
│  ├─ Virtual mapping (user space)
│  ├─ Lazy allocation
│  └─ Copy-on-write
└─ Protection Bits
   ├─ Read/Write/Execute
   ├─ User/Supervisor
   └─ Caching modes
```

**Tests Required**:
- Frame allocation/deallocation
- Page table walking
- Permission enforcement
- Fault recovery

### 2. Heap Allocator (`src/phase2/allocator.rs`)
**Current Status**: Placeholder with `BumpAllocator` structure

**Planned Implementation**:
```
├─ BumpAllocator (Phase 2a)
│  ├─ Simple linear allocation
│  ├─ No deallocation support
│  └─ ~2MB for Phase 2-3
├─ Linked List Allocator (Phase 2b)
│  ├─ Free list management
│  ├─ Coalescence
│  └─ ~50MB for Phase 4+
└─ Advanced Features (Phase 3+)
   ├─ Slab allocator
   ├─ Virtual allocation
   └─ NUMA support
```

**Memory Regions**:
- Kernel heap: `0x_4444_4444_0000` (100 MB initially)
- User heap: `0x_6000_0000_0000` (varies, lazy)

**Allocation Statistics**:
- Track allocated/free bytes
- Fragmentation ratio
- Cache statistics

### 3. Graphics Driver (`src/phase2/graphics.rs`)
**Current Status**: Framework with video mode enumeration

**Planned Implementation**:
```
├─ VESA Detection
│  ├─ Real-mode BIOS calls
│  ├─ Mode enumeration
│  ├─ Capabilities detection
│  └─ Mode switching
├─ Framebuffer Management
│  ├─ Linear framebuffer
│  ├─ MMIO regions
│  ├─ Double buffering
│  └─ Vsync support
├─ Rendering Functions
│  ├─ set_pixel(x, y, color)
│  ├─ draw_rect(x, y, w, h, color)
│  ├─ draw_line(x0, y0, x1, y1, color)
│  ├─ fill_screen(color)
│  ├─ draw_char(x, y, char, fg, bg)
│  └─ blit(src_fb, dst_fb, region)
├─ Color Management
│  ├─ 32-bit ARGB
│  ├─ Blending
│  └─ Color space conversion
└─ Hardware Cursor
   ├─ Shape definition
   ├─ Position update
   └─ Hotspot support
```

**Supported Modes** (Initial):
- 1024x768@60Hz (default)
- 1280x1024@60Hz
- 1920x1080@60Hz
- All 32-bit color (ARGB)

## Implementation Roadmap

### Week 1: Memory Paging
- [ ] Implement page table structures
- [ ] Add page walk algorithm
- [ ] Create page fault handler
- [ ] Test with kernel memory

### Week 2: Heap Allocator
- [ ] Implement bump allocator (working)
- [ ] Create linked list allocator
- [ ] Add allocation statistics
- [ ] Stress test under allocation pressure

### Week 3-4: Graphics
- [ ] Detect available modes via VESA
- [ ] Implement mode switching
- [ ] Set up framebuffer
- [ ] Create rendering primitives
- [ ] Display NIVA desktop boot screen

## Integration Points

### With Phase 1:
- Reuse bootloader's initial page tables
- Keep GDT/IDT from Phase 1
- Extend IDT for page faults

### With Phase 3 (Filesystem):
- Phase 2 allocator enables dynamic file caching
- Graphics output for file browser
- Virtual memory for large files

### With Phase 4 (Desktop):
- Graphics enables window rendering
- Allocator needed for window/widget objects
- Virtual memory for app isolation

## Memory Layout After Phase 2

```
Virtual Address Space (64-bit):
┌─────────────────────────────────────┐
│  User Space (Ring 3)                │ 0x0000_0000_0000_0000 - 0x0000_7FFF_FFFF_FFFF
│  - Stack (grows down)               │
│  - Heap (grows up)                  │
│  - Shared libraries                 │
│  - Memory-mapped I/O                │
├─────────────────────────────────────┤
│  Kernel Space (Ring 0)              │ 0xFFFF_8000_0000_0000 - 0xFFFF_FFFF_FFFF_FFFF
│  - Kernel code/data                 │ 0xFFFF_8000_0100_0000
│  - Kernel heap                      │ 0xFFFF_8000_0200_0000 (50 MB)
│  - Page tables                      │ 0xFFFF_8000_0400_0000
│  - MMIO regions                     │ 0xFFFF_8000_1000_0000
│  - Framebuffer                      │ 0xFFFF_8000_2000_0000
│  - Stack (kernel)                   │ 0xFFFF_8000_3000_0000
└─────────────────────────────────────┘
```

## Success Criteria

✓ Phase 2 is complete when:
- [x] Framework compiles without errors
- [ ] Paging system enables/disables correctly
- [ ] Heap allocator survives allocation pressure
- [ ] Graphics mode switches to 1024x768
- [ ] Framebuffer renders pixels
- [ ] Boot output displays on desktop
- [ ] All tests pass
- [ ] Documentation updated

## Testing Strategy

### Unit Tests
- Frame allocator operations
- Heap allocation/deallocation
- Page table walking
- Color operations

### Integration Tests
- Boot with paging enabled
- Allocate/free under load
- Switch graphics modes
- Render boot screen

### Stress Tests
- Allocate maximum heap
- Fault handling under load
- Large memory transfers
- Rapid mode switching

## Known Issues / TODOs

### Memory
- [ ] Implement actual page table manipulation
- [ ] Add page fault recovery
- [ ] Implement free list coalescing
- [ ] Add memory statistics syscalls

### Graphics
- [ ] Implement VESA BIOS calls
- [ ] Add VGA text mode fallback
- [ ] Create font rendering
- [ ] Add sprite/bitmap support

### Performance
- [ ] TLB flushing optimization
- [ ] Allocator benchmarking
- [ ] Cache-line alignment
- [ ] Memory access profiling

## Phase 2 Features at Completion

**Operational**:
- Virtual address translation (any 64-bit address)
- Kernel memory allocation/deallocation
- Graphics mode detection and switching
- Pixel-level rendering
- Boot screen display

**For Applications**:
- Individual address spaces (foundation)
- Dynamic memory allocation
- Visual I/O

**For System**:
- Memory protection boundaries
- Framebuffer for desktop rendering
- Allocator statistics
- Page fault handling

## Performance Targets

- Frame allocation: < 1 µs
- Memory allocation (small): < 10 µs
- Mode switching: < 100 ms
- Framebuffer clear: < 50 ms (1920x1080)
- Graphics latency: < 16 ms (60 FPS)

## Dependencies & Prerequisites

**Required for Phase 2**:
- Phase 1 (bootloader, kernel, exceptions) ✓
- x86_64 interrupt support ✓
- GDT/IDT from Phase 1 ✓

**Enabled by Phase 2**:
- Phase 3: Filesystem (needs allocator)
- Phase 4: Desktop (needs graphics)
- Phase 5: Processes (needs paging)

---

**Phase 2 Status**: Initiated  
**Estimated Duration**: 3-4 weeks  
**Starting Date**: After Phase 1 completion  
**Target Completion**: Graphics boot screen rendering  
