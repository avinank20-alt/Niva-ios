// Memory management

pub mod allocator;
pub mod paging;

pub fn init() {
    crate::println!("[*] Initializing memory manager...");
    allocator::init();
    paging::init();
}
