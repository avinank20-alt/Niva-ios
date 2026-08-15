// Device drivers

pub mod pic;
pub mod keyboard;
pub mod timer;
pub mod graphics;

pub fn init_all() {
    crate::println!("[*] Initializing device drivers...");
    pic::init();
}
