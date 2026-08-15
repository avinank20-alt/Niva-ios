// Virtual memory paging

pub struct PageTable;

impl PageTable {
    pub fn new() -> Self {
        PageTable
    }

    pub fn map(&mut self, virtual_addr: u64, physical_addr: u64) {
        crate::println!("[*] Mapping {:x} -> {:x}", virtual_addr, physical_addr);
    }

    pub fn unmap(&mut self, virtual_addr: u64) {
        crate::println!("[*] Unmapping {:x}", virtual_addr);
    }
}

pub fn init() {
    crate::println!("[*] Paging initialized");
}
