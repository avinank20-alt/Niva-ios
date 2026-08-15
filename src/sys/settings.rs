// System Settings and Configuration

#[derive(Clone, Debug)]
pub struct SystemSettings {
    pub theme: &'static str,
    pub language: &'static str,
    pub brightness: u8,
    pub volume: u8,
    pub auto_update: bool,
    pub privacy_mode: bool,
}

impl Default for SystemSettings {
    fn default() -> Self {
        SystemSettings {
            theme: "dark",
            language: "en_US",
            brightness: 100,
            volume: 50,
            auto_update: true,
            privacy_mode: false,
        }
    }
}

pub struct SettingsManager {
    settings: SystemSettings,
}

impl SettingsManager {
    pub fn new() -> Self {
        SettingsManager {
            settings: SystemSettings::default(),
        }
    }

    pub fn get_setting(&self, key: &str) -> &str {
        match key {
            "theme" => self.settings.theme,
            "language" => self.settings.language,
            _ => "unknown",
        }
    }

    pub fn set_theme(&mut self, theme: &'static str) {
        self.settings.theme = theme;
        crate::println!("[*] Theme changed to: {}", theme);
    }

    pub fn set_privacy_mode(&mut self, enabled: bool) {
        self.settings.privacy_mode = enabled;
        crate::println!("[*] Privacy mode: {}", if enabled { "ON" } else { "OFF" });
    }
}

pub fn init() {
    crate::println!("[*] Settings manager initialized");
}
