// UI Theme system

#[derive(Clone, Copy, Debug)]
pub struct Color(pub u32);

pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Theme {
            primary: Color(0x1e1e2e),      // Very dark blue-gray
            secondary: Color(0x313244),    // Dark blue-gray
            background: Color(0x1e1e2e),
            foreground: Color(0xcdd6f4),   // Light text
            accent: Color(0x89b4fa),       // Blue accent
        }
    }

    pub fn light() -> Self {
        Theme {
            primary: Color(0xf5f5f5),
            secondary: Color(0xe8e8e8),
            background: Color(0xffffff),
            foreground: Color(0x1e1e2e),
            accent: Color(0x0078d4),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::dark()
    }
}
