// Keyboard driver

pub struct KeyboardDriver;

impl KeyboardDriver {
    pub fn new() -> Self {
        KeyboardDriver
    }

    pub fn init() {
        crate::println!("[*] Keyboard driver initialized");
    }
}
