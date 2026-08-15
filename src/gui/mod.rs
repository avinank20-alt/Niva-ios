// Graphics User Interface (GUI) and Desktop Environment

pub mod window;
pub mod desktop;
pub mod taskbar;
pub mod theme;

pub fn init() {
    crate::println!("[*] Initializing GUI system...");
    desktop::init();
    taskbar::init();
}
