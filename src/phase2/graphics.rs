// Graphics Driver
// Implements VESA graphics mode and framebuffer rendering

use crate::println;

/// Video mode information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoMode {
    pub width: u32,
    pub height: u32,
    pub bpp: u8, // bits per pixel
    pub refresh_rate: u8,
}

impl VideoMode {
    /// Get mode name for display
    pub fn name(&self) -> &'static str {
        match (self.width, self.height) {
            (1024, 768) => "XGA",
            (1280, 1024) => "SXGA",
            (1920, 1080) => "FHD",
            _ => "Custom",
        }
    }

    /// Get memory required for framebuffer
    pub fn framebuffer_size(&self) -> usize {
        (self.width * self.height * (self.bpp as u32 / 8)) as usize
    }
}

/// Color representation (ARGB)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u32);

impl Color {
    pub const BLACK: Color = Color(0xFF000000);
    pub const WHITE: Color = Color(0xFFFFFFFF);
    pub const RED: Color = Color(0xFFFF0000);
    pub const GREEN: Color = Color(0xFF00FF00);
    pub const BLUE: Color = Color(0xFF0000FF);
    pub const GRAY: Color = Color(0xFF808080);
    pub const TRANSPARENT: Color = Color(0x00000000);

    /// Create RGB color
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color(0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    /// Create RGBA color
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
                if x + dx < self.width && y + dy < self.height {
                    self.set_pixel(x + dx, y + dy, color);
                }
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
            if x + i < self.width {
                self.set_pixel(x + i, y, color);
            }
        }
    }

    /// Draw a vertical line
    pub fn draw_vline(&self, x: u32, y: u32, length: u32, color: Color) {
        for i in 0..length {
            if y + i < self.height {
                self.set_pixel(x, y + i, color);
            }
        }
    }

    /// Draw a frame (hollow rectangle)
    pub fn draw_frame(&self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        // Top and bottom
        self.draw_hline(x, y, width, color);
        self.draw_hline(x, y + height - 1, width, color);
        
        // Left and right
        self.draw_vline(x, y, height, color);
        self.draw_vline(x + width - 1, y, height, color);
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

    /// Get available video modes
    pub fn available_modes() -> &'static [VideoMode] {
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

    /// Detect available video modes
    pub fn detect_modes() -> &'static [VideoMode] {
        Graphics::available_modes()
    }

    /// Set video mode
    pub fn set_mode(&mut self, mode: VideoMode) -> Result<(), &'static str> {
        println!("[*] Setting video mode: {}x{}@{}Hz ({} bpp)", 
                 mode.width, mode.height, mode.refresh_rate, mode.bpp);
        self.current_mode = Some(mode);
        Ok(())
    }

    /// Get current mode
    pub fn current_mode(&self) -> Option<VideoMode> {
        self.current_mode
    }

    /// Get framebuffer
    pub fn framebuffer(&self) -> Option<&Framebuffer> {
        self.framebuffer.as_ref()
    }
}

/// Global graphics instance (will be initialized during boot)
pub static mut GRAPHICS: Graphics = Graphics::new();

/// Initialize graphics subsystem
pub fn init_graphics() {
    println!("[*] Initializing graphics subsystem...");
    
    let modes = Graphics::detect_modes();
    println!("    Available video modes:");
    
    for (i, mode) in modes.iter().enumerate() {
        println!(
            "    [{}] {} ({}x{}@{}Hz, {} bpp, {} MB buffer)",
            i,
            mode.name(),
            mode.width,
            mode.height,
            mode.refresh_rate,
            mode.bpp,
            mode.framebuffer_size() / 1024 / 1024
        );
    }
    
    println!();
    println!("    Status: Graphics driver framework loaded");
    println!("    Note: VESA mode switching requires real-mode BIOS calls");
    
    // TODO: Implement:
    // - Real-mode BIOS calls for VESA detection
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
        assert_eq!(modes[0].width, 1024);
        assert_eq!(modes[0].height, 768);
    }

    #[test]
    fn test_video_mode_name() {
        let mode = VideoMode {
            width: 1024,
            height: 768,
            bpp: 32,
            refresh_rate: 60,
        };
        assert_eq!(mode.name(), "XGA");
    }

    #[test]
    fn test_framebuffer_size() {
        let mode = VideoMode {
            width: 1024,
            height: 768,
            bpp: 32,
            refresh_rate: 60,
        };
        // 1024 * 768 * 4 bytes = 3,145,728 bytes
        assert_eq!(mode.framebuffer_size(), 1024 * 768 * 4);
    }
}
