// ext4 Filesystem driver

pub struct Ext4FileSystem {
    pub block_size: u32,
    pub inode_size: u32,
}

impl Ext4FileSystem {
    pub fn new() -> Self {
        Ext4FileSystem {
            block_size: 4096,
            inode_size: 256,
        }
    }

    pub fn init(&mut self) {
        crate::println!("[*] Ext4 filesystem initialized");
        crate::println!("    Block size: {} bytes", self.block_size);
        crate::println!("    Inode size: {} bytes", self.inode_size);
    }
}
