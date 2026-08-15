// Graphics Driver
// Implements VESA graphics mode and framebuffer rendering

use crate::println;

/// Video mode information
#[derive(Debug, Clone, Copy)]
pub struct VideoMode {
    pub width: u32,
    pub height: u32,
    pub bpp: u8, // bits per pixel
    pub refresh_rate: u8,
}

/// Color representation (ARGB)
#[derive(Debug, Clone, Copy)]
pub struct Color(pub u32);

impl Color {
    pub const BLACK: Color = Color(0xFF000000);
    pub const WHITE: Color = Color(0xFFFFFFFF);
    pub const RED: Color = Color(0xFFFF0000);
    pub const GREEN: Color = Color(0xFF00FF00);
    pub const BLUE: Color = Color(0xFF0000FF);
    pub const TRANSPARENT: Color = Color(0x00000000);

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color(0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }
}

/// Framebuffer interface
pub struct Framebuffer {
    address: *mut u32,
    width: u32,
    height: u32,
    pitch: u32, // bytes per line
}

impl Framebuffer {
    /// Create a new framebuffer
    pub fn new(address: *mut u32, width: u32, height: u32, pitch: u32) -> Self {
        Self {
            address,
            width,
            height,
            pitch,
        }
    }

    /// Set a pixel at the given coordinates
    pub fn set_pixel(&self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        unsafe {
            let offset = (y * self.pitch / 4) + x;
            *self.address.add(offset as usize) = color.0;
        }
    }

    /// Draw a filled rectangle
    pub fn draw_rect(&self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        for dy in 0..height {
            for dx in 0..width {
                self.set_pixel(x + dx, y + dy, color);
            }
        }
    }

    /// Clear the entire framebuffer to a color
    pub fn clear(&self, color: Color) {
        self.draw_rect(0, 0, self.width, self.height, color);
    }

    /// Draw a horizontal line
    pub fn draw_hline(&self, x: u32, y: u32, length: u32, color: Color) {
        for i in 0..length {
            self.set_pixel(x + i, y, color);
        }
    }

    /// Draw a vertical line
    pub fn draw_vline(&self, x: u32, y: u32, length: u32, color: Color) {
        for i in 0..length {
            self.set_pixel(x, y + i, color);
        }
    }

    /// Get width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> u32 {
        self.height
    }
}

/// Graphics subsystem
pub struct Graphics {
    current_mode: Option<VideoMode>,
    framebuffer: Option<Framebuffer>,
}

impl Graphics {
    pub const fn new() -> Self {
        Self {
            current_mode: None,
            framebuffer: None,
        }
    }

    /// Detect available video modes
    pub fn detect_modes() -> &'static [VideoMode] {
        // TODO: Query VESA BIOS for available modes
        // For now, return common modes
        &[
            VideoMode {
                width: 1024,
                height: 768,
                bpp: 32,
                refresh_rate: 60,
            },
            VideoMode {
                width: 1280,
                height: 1024,
                bpp: 32,
                refresh_rate: 60,
            },
            VideoMode {
                width: 1920,
                height: 1080,
                bpp: 32,
                refresh_rate: 60,
            },
        ]
    }

    /// Set video mode
    pub fn set_mode(&mut self, mode: VideoMode) {
        println!("[*] Setting video mode: {}x{}@{}", mode.width, mode.height, mode.bpp);
        // TODO: Implement VESA mode setting
        self.current_mode = Some(mode);
    }
}

/// Global graphics instance (will be initialized during boot)
pub static mut GRAPHICS: Graphics = Graphics::new();

/// Initialize graphics subsystem
pub fn init_graphics() {
    println!("[*] Initializing graphics subsystem...");
    
    let modes = Graphics::detect_modes();
    println!("    Available video modes: {}", modes.len());
    
    for mode in modes {
        println!(
            "    - {}x{}@{}Hz ({} bpp)",
            mode.width, mode.height, mode.refresh_rate, mode.bpp
        );
    }
    
    // TODO: Implement:
    // - VESA mode detection
    // - Mode switching
    // - Framebuffer mapping
    // - Hardware cursor support
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let red = Color::RED;
        assert_eq!(red.0, 0xFFFF0000);
        
        let custom = Color::rgb(255, 128, 64);
        assert_eq!(custom.0, 0xFFFF8040);
    }

    #[test]
    fn test_video_modes() {
        let modes = Graphics::detect_modes();
        assert!(!modes.is_empty());
    }
}
