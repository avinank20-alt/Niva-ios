// Memory management for x86_64

use x86_64::registers::control::Cr3;
use x86_64::structures::paging::PageTable;
use x86_64::VirtAddr;

/// Get current page table
pub fn get_page_table() -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = VirtAddr::new(phys.as_u64() + 0xffff800000000000);
    unsafe { &mut *(virt.as_mut_ptr() as *mut PageTable) }
}

/// Initialize memory management
pub fn init() {
    crate::println!("[*] Initializing memory management...");
}
