// Filesystem implementation

pub mod vfs;
pub mod ext4;

pub fn init() {
    crate::println!("[*] Initializing filesystem...");
    vfs::init();
}
