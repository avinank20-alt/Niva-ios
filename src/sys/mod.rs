// System services and applications

pub mod app_manager;
pub mod settings;
pub mod file_manager;
pub mod browser;

pub fn init() {
    crate::println!("[*] Initializing system services...");
    app_manager::init();
    settings::init();
}
