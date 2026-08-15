// Heap Memory Allocator - Bump allocator for kernel heap

use crate::println;
#[derive(Debug)]
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next_alloc: usize,
}

impl BumpAllocator {
    /// Create a new bump allocator
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        Self {
            heap_start,
            heap_end: heap_start + heap_size,
            next_alloc: heap_start,
        }
    }

    /// Allocate bytes from the heap (simple linear allocation)
    pub fn allocate(&mut self, size: usize, alignment: usize) -> Option<*mut u8> {
        // Align the next allocation address
        let aligned = (self.next_alloc + alignment - 1) & !(alignment - 1);
        
        if aligned + size <= self.heap_end {
            self.next_alloc = aligned + size;
            Some(aligned as *mut u8)
        } else {
            None
        }
    }

    /// Get the number of allocated bytes
    pub fn allocated_bytes(&self) -> usize {
        self.next_alloc - self.heap_start
    }

    /// Get the total heap size
    pub fn total_bytes(&self) -> usize {
        self.heap_end - self.heap_start
    }

    /// Get remaining heap space
    pub fn remaining_bytes(&self) -> usize {
        self.heap_end - self.next_alloc
    }

    /// Get allocation statistics
    pub fn stats(&self) -> (usize, usize, usize) {
        (self.allocated_bytes(), self.remaining_bytes(), self.total_bytes())
    }
}

/// Global heap allocator instance (bump allocator used for now)
pub static mut GLOBAL_ALLOCATOR: Option<BumpAllocator> = None;

/// Initialize the heap allocator
pub fn init_heap() {
    println!("[*] Initializing kernel heap allocator...");
    
    const HEAP_START: usize = 0xffff_8000_0200_0000;
    const HEAP_SIZE: usize = 100 * 1024 * 1024; // 100 MB

    let allocator = BumpAllocator::new(HEAP_START, HEAP_SIZE);
    
    unsafe {
        GLOBAL_ALLOCATOR = Some(allocator);
    }
    
    println!(
        "    Heap: 0x{:x} - 0x{:x}",
        HEAP_START,
        HEAP_START + HEAP_SIZE
    );
    println!(
        "    Size: {} MB available",
        HEAP_SIZE / 1024 / 1024
    );
    println!("    Type: Bump allocator (linear, no deallocation)");
    println!("    Note: Linked list allocator framework ready for Phase 2b");
    
    // Get stats
    unsafe {
        if let Some(alloc) = &GLOBAL_ALLOCATOR {
            let (allocated, remaining, total) = alloc.stats();
            println!(
                "    Status: {} bytes allocated, {} bytes remaining (total {} bytes)",
                allocated, remaining, total
            );
        }
    }
}

/// Allocate memory from the global heap
#[allow(dead_code)]
pub fn allocate(size: usize, alignment: usize) -> Option<*mut u8> {
    unsafe {
        if let Some(alloc) = &mut GLOBAL_ALLOCATOR {
            alloc.allocate(size, alignment)
        } else {
            None
        }
    }
}

/// Deallocate memory from the heap
#[allow(dead_code)]
pub unsafe fn deallocate(ptr: *mut u8, size: usize) {
    // Bump allocator doesn't support deallocation
    // This would be implemented in linked list allocator
    let _ = (ptr, size);
}

/// Get heap statistics
#[allow(dead_code)]
pub fn heap_stats() -> Option<(usize, usize, usize)> {
    unsafe {
        if let Some(alloc) = &GLOBAL_ALLOCATOR {
            Some(alloc.stats())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bump_allocator() {
        let mut allocator = BumpAllocator::new(0x1000, 0x1000);
        assert_eq!(allocator.allocated_bytes(), 0);
        assert_eq!(allocator.total_bytes(), 0x1000);
        
        let ptr = allocator.allocate(256, 8);
        assert!(ptr.is_some());
        assert_eq!(allocator.allocated_bytes(), 256);
    }

    #[test]
    fn test_bump_allocator_alignment() {
        let mut allocator = BumpAllocator::new(0x1000, 0x1000);
        
        // Allocate with 16-byte alignment
        let ptr1 = allocator.allocate(8, 16);
        assert!(ptr1.is_some());
        
        let addr = ptr1.unwrap() as usize;
        assert_eq!(addr % 16, 0, "Allocation should be 16-byte aligned");
    }

    #[test]
    fn test_bump_allocator_exhaustion() {
        let mut allocator = BumpAllocator::new(0x1000, 256);
        
        // Should succeed
        let ptr1 = allocator.allocate(128, 4);
        assert!(ptr1.is_some());
        
        // Should also succeed
        let ptr2 = allocator.allocate(128, 4);
        assert!(ptr2.is_some());
        
        // Should fail (not enough space)
        let ptr3 = allocator.allocate(256, 4);
        assert!(ptr3.is_none());
    }

    #[test]
    fn test_linked_list_allocator() {
        let mut allocator = LinkedListAllocator::new();
        assert!(!allocator.is_initialized());
        
        unsafe {
            allocator.init(0x10000, 0x10000);
        }
        assert!(allocator.is_initialized());
    }

    #[test]
    fn test_linked_list_allocator_stats() {
        let mut allocator = LinkedListAllocator::new();
        unsafe {
            allocator.init(0x10000, 0x1000);
        }
        
        let (allocated, remaining, total) = allocator.stats();
        assert_eq!(total, 0x1000);
        assert_eq!(allocated, 0);
    }
}
