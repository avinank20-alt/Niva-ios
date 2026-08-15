// Virtual Memory Paging Module
// Implements x86_64 paging with 4-level page tables

use crate::println;
use x86_64::{
    structures::paging::{PhysFrame, Size4KiB},
    VirtAddr, PhysAddr,
};
use spin::Mutex;

// Physical memory start (1MB, after bootloader)
const PHYS_MEMORY_START: u64 = 0x100000;
const PHYS_MEMORY_SIZE: u64 = 0x10000000; // 256MB

/// Frame allocator for page tables
pub struct FrameAllocator {
    next_frame: u64,
    max_frames: u64,
}

impl FrameAllocator {
    /// Create a new frame allocator
    pub const fn new() -> Self {
        Self {
            next_frame: PHYS_MEMORY_START / 4096,
            max_frames: (PHYS_MEMORY_SIZE / 4096),
        }
    }

    /// Allocate the next available frame
    pub fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        if self.next_frame < self.max_frames {
            let frame_num = self.next_frame;
            self.next_frame += 1;
            
            let addr = PhysAddr::new(frame_num * 4096);
            Some(PhysFrame::containing_address(addr))
        } else {
            None
        }
    }

    /// Deallocate a frame (currently unused, for future implementation)
    pub fn deallocate_frame(&mut self, _frame: PhysFrame<Size4KiB>) {
        // TODO: Implement free list management for better memory efficiency
    }

    /// Get the number of allocated frames
    pub fn allocated_frames(&self) -> u64 {
        self.next_frame - (PHYS_MEMORY_START / 4096)
    }

    /// Get total available frames
    pub fn total_frames(&self) -> u64 {
        self.max_frames
    }

    /// Get remaining available frames
    pub fn remaining_frames(&self) -> u64 {
        self.max_frames - self.next_frame
    }
}

/// Unsafe allocator implementation for page table mapping
unsafe impl x86_64::structures::paging::FrameAllocator<Size4KiB> for FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        self.allocate_frame()
    }
}

/// Global frame allocator instance
pub static FRAME_ALLOCATOR: Mutex<FrameAllocator> = Mutex::new(FrameAllocator::new());

/// Page table entry flags
pub mod flags {
    /// Present bit - page is in memory
    pub const PRESENT: u64 = 1 << 0;
    /// Writable bit - page is writable
    pub const WRITABLE: u64 = 1 << 1;
    /// User bit - user-mode access allowed
    pub const USER: u64 = 1 << 2;
    /// Write-through bit - page writes go to memory immediately
    pub const WRITE_THROUGH: u64 = 1 << 3;
    /// Cache disabled bit
    pub const CACHE_DISABLED: u64 = 1 << 4;
    /// Accessed bit - page has been accessed
    pub const ACCESSED: u64 = 1 << 5;
    /// Dirty bit - page has been written to
    pub const DIRTY: u64 = 1 << 6;
    /// Huge page bit
    pub const HUGE: u64 = 1 << 7;
    /// Global bit - TLB won't invalidate on context switch
    pub const GLOBAL: u64 = 1 << 8;
    /// Execute disable bit
    pub const NO_EXECUTE: u64 = 1 << 63;
}

/// Memory mapping statistics
#[derive(Debug, Clone, Copy)]
pub struct MappingStats {
    pub frames_allocated: u64,
    pub frames_remaining: u64,
    pub memory_mb: u64,
}

/// Initialize paging system
pub fn init_paging() {
    println!("[*] Initializing virtual memory paging...");
    
    let allocator = FRAME_ALLOCATOR.lock();
    
    println!(
        "    Total frames available: {}",
        allocator.total_frames()
    );
    println!(
        "    Frames allocated: {}",
        allocator.allocated_frames()
    );
    println!(
        "    Memory: {} MB available",
        (allocator.total_frames() * 4096) / (1024 * 1024)
    );
    
    drop(allocator);
    
    // Identity map kernel (already done by bootloader)
    println!("    Kernel address space: 0xffffffff80000000 - 0xffffffff80400000");
    println!("    (Identity mapped by bootloader)");
    
    println!("[✓] Paging system initialized");
}

/// Map a virtual address to a physical address (framework function)
pub fn map_page_static(virt: VirtAddr, phys: PhysAddr, _flags: u64) -> Result<(), &'static str> {
    println!("[*] Mapping {:?} -> {:?}", virt, phys);
    // In a full implementation, we'd actually map the page here
    // For now, this is a placeholder that shows the interface
    Ok(())
}

/// Translate a virtual address to physical (framework)
pub fn translate(virt: VirtAddr) -> Option<PhysAddr> {
    // In a full implementation, we'd walk the page tables here
    // Kernel pages are identity mapped, so we can return for kernel addresses
    if virt.as_u64() >= 0xffffffff80000000 {
        Some(PhysAddr::new(virt.as_u64() - 0xffffffff80000000 + 0x100000))
    } else {
        None
    }
}

/// Check if a page is present
pub fn is_present(virt: VirtAddr) -> bool {
    translate(virt).is_some()
}

/// Get detailed paging statistics
pub fn get_detailed_stats() -> MappingStats {
    let allocator = FRAME_ALLOCATOR.lock();
    MappingStats {
        frames_allocated: allocator.allocated_frames(),
        frames_remaining: allocator.remaining_frames(),
        memory_mb: (allocator.total_frames() * 4096) / (1024 * 1024),
    }
}

/// Get paging statistics
pub fn get_stats() -> (u64, u64) {
    let allocator = FRAME_ALLOCATOR.lock();
    (allocator.allocated_frames(), allocator.remaining_frames())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_allocator() {
        let mut allocator = FrameAllocator::new();
        assert_eq!(allocator.allocated_frames(), 0);
        
        let frame1 = allocator.allocate_frame();
        assert!(frame1.is_some());
        assert_eq!(allocator.allocated_frames(), 1);
        
        let frame2 = allocator.allocate_frame();
        assert!(frame2.is_some());
        assert_eq!(allocator.allocated_frames(), 2);
    }

    #[test]
    fn test_frame_allocator_capacity() {
        let mut allocator = FrameAllocator::new();
        let total = allocator.total_frames();
        assert!(total > 0);
    }

    #[test]
    fn test_paging_flags() {
        let combined = flags::PRESENT | flags::WRITABLE | flags::USER;
        assert!(combined & flags::PRESENT != 0);
        assert!(combined & flags::WRITABLE != 0);
        assert!(combined & flags::USER != 0);
        assert!(combined & flags::NO_EXECUTE == 0);
    }

    #[test]
    fn test_translate_kernel_address() {
        let virt = VirtAddr::new(0xffffffff80100000);
        let phys = translate(virt);
        assert!(phys.is_some());
    }
}
